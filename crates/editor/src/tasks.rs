use crate::Editor;

use gpui::{Task as AsyncTask, WindowContext};
use project::Location;
use task::{TaskContext, VariableName};
use text::Point;
use util::ResultExt;
use workspace::Workspace;

fn task_context_with_editor(
    editor: &mut Editor,
    cx: &mut WindowContext<'_>,
) -> AsyncTask<Option<TaskContext>> {
    let Some(project) = editor.project.clone() else {
        return AsyncTask::ready(None);
    };
    let (selection, buffer, editor_snapshot) = {
        let mut selection = editor.selections.newest::<Point>(cx);
        if editor.selections.line_mode {
            selection.start = Point::new(selection.start.row, 0);
            selection.end = Point::new(selection.end.row + 1, 0);
        }
        let Some((buffer, _, _)) = editor
            .buffer()
            .read(cx)
            .point_to_buffer_offset(selection.start, cx)
        else {
            return AsyncTask::ready(None);
        };
        let snapshot = editor.snapshot(cx);
        (selection, buffer, snapshot)
    };
    let selection_range = selection.range();
    let start = editor_snapshot
        .display_snapshot
        .buffer_snapshot
        .anchor_after(selection_range.start)
        .text_anchor;
    let end = editor_snapshot
        .display_snapshot
        .buffer_snapshot
        .anchor_after(selection_range.end)
        .text_anchor;
    let location = Location {
        buffer,
        range: start..end,
    };

    let context_task = project.update(cx, |project, cx| {
        project.task_context_for_location(location.clone(), cx)
    });
    cx.spawn(|mut cx| async move {
        context_task.await.map(|mut task_context| {
            let Some(buffer_snapshot) = cx
                .update(|cx| location.buffer.read(cx).snapshot())
                .log_err()
            else {
                return task_context;
            };
            for range in buffer_snapshot.runnable_ranges(location.range) {
                for (capture_name, value) in range.extra_captures {
                    task_context
                        .task_variables
                        .insert(VariableName::Custom(capture_name.into()), value);
                }
            }
            task_context
        })
    })
}

pub fn task_context(workspace: &Workspace, cx: &mut WindowContext<'_>) -> AsyncTask<TaskContext> {
    let Some(editor) = workspace
        .active_item(cx)
        .and_then(|item| item.act_as::<Editor>(cx))
    else {
        return AsyncTask::ready(TaskContext::default());
    };
    editor.update(cx, |editor, cx| {
        let context_task = task_context_with_editor(editor, cx);
        cx.background_executor()
            .spawn(async move { context_task.await.unwrap_or_default() })
    })
}
