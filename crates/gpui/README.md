# Welcome to GPUI!

GPUI is a hybrid immediate and retained mode, GPU accelerated, UI framework
for Rust, designed to support a wide variety of applications.

## Getting Started

GPUI is still in active development as we work on the Zed code editor, and is still pre-1.0. There will often be breaking changes between versions. You'll also need to use the latest version of stable Rust and be on macOS or Linux. Add the following to your `Cargo.toml`:

```toml
gpui = { version = "*" }
```

 - [Ownership and data flow](_ownership_and_data_flow)

Everything in GPUI starts with an `Application`. You can create one with `Application::new()`, and kick off your application by passing a callback to `Application::run()`. Inside this callback, you can create a new window with `App::open_window()`, and register your first root view. See [gpui.rs](https://www.gpui.rs/) for a complete example.

## Embedding GPUI (Electron, host-owned UI loop)

GPUI can be used in a process where another host already initialized and is already pumping the OS UI loop (for example, an Electron app). For this scenario, use `Application::run_embedded`, which does not take ownership of the platform event loop.

```rust
use gpui::Application;

fn init_gpui() {
    let _app = Application::new().run_embedded(|cx| {
        // Configure GPUI and open windows here.
        //
        // Keep the returned handle alive for as long as you want GPUI to stay initialized.
    });
}
```

### Platform notes

- macOS: `run_embedded` installs a GPUI NSApp delegate proxy that forwards unknown selectors to the host’s original delegate, so GPUI can continue to receive menu, dock-menu, reopen, and open-URL callbacks while preserving host behavior.
- Windows: `run_embedded` starts GPUI’s vsync/invalidation thread. The host must pump the Win32 message loop on the same thread that created the GPUI app so `WM_GPUI_*` messages are dispatched.
- Linux: `run_embedded` does not currently integrate GPUI’s calloop-based event loop into a host loop. The recommended approach is to run GPUI on a dedicated thread (same process) and use the normal `Application::run` on that thread.

### Capability matrix (embedded mode)

| Platform | Owns event loop? | Typical requirement | App-level callbacks |
| --- | --- | --- | --- |
| macOS | No | Must run on the macOS main thread | Preserved via delegate proxy |
| Windows | No | Host pumps message loop on same thread | Preserved via message-only HWND + forwarding |
| Linux | Not integrated | Run GPUI on a dedicated thread | Works when GPUI owns its calloop loop |

### Dependencies

GPUI has various system dependencies that it needs in order to work.

#### macOS

On macOS, GPUI uses Metal for rendering. In order to use Metal, you need to do the following:

- Install [Xcode](https://apps.apple.com/us/app/xcode/id497799835?mt=12) from the macOS App Store, or from the [Apple Developer](https://developer.apple.com/download/all/) website. Note this requires a developer account.

> Ensure you launch Xcode after installing, and install the macOS components, which is the default option.

- Install [Xcode command line tools](https://developer.apple.com/xcode/resources/)

  ```sh
  xcode-select --install
  ```

- Ensure that the Xcode command line tools are using your newly installed copy of Xcode:

  ```sh
  sudo xcode-select --switch /Applications/Xcode.app/Contents/Developer
  ```

## The Big Picture

GPUI offers three different [registers](<https://en.wikipedia.org/wiki/Register_(sociolinguistics)>) depending on your needs:

- State management and communication with `Entity`'s. Whenever you need to store application state that communicates between different parts of your application, you'll want to use GPUI's entities. Entities are owned by GPUI and are only accessible through an owned smart pointer similar to an `Rc`. See the `app::context` module for more information.

- High level, declarative UI with views. All UI in GPUI starts with a view. A view is simply an `Entity` that can be rendered, by implementing the `Render` trait. At the start of each frame, GPUI will call this render method on the root view of a given window. Views build a tree of `elements`, lay them out and style them with a tailwind-style API, and then give them to GPUI to turn into pixels. See the `div` element for an all purpose swiss-army knife of rendering.

- Low level, imperative UI with Elements. Elements are the building blocks of UI in GPUI, and they provide a nice wrapper around an imperative API that provides as much flexibility and control as you need. Elements have total control over how they and their child elements are rendered and can be used for making efficient views into large lists, implement custom layouting for a code editor, and anything else you can think of. See the `element` module for more information.

Each of these registers has one or more corresponding contexts that can be accessed from all GPUI services. This context is your main interface to GPUI, and is used extensively throughout the framework.

## Other Resources

In addition to the systems above, GPUI provides a range of smaller services that are useful for building complex applications:

- Actions are user-defined structs that are used for converting keystrokes into logical operations in your UI. Use this for implementing keyboard shortcuts, such as cmd-q. See the `action` module for more information.

- Platform services, such as `quit the app` or `open a URL` are available as methods on the `app::App`.

- An async executor that is integrated with the platform's event loop. See the `executor` module for more information.,

- The `[gpui::test]` macro provides a convenient way to write tests for your GPUI applications. Tests also have their own kind of context, a `TestAppContext` which provides ways of simulating common platform input. See `app::test_context` and `test` modules for more details.

Currently, the best way to learn about these APIs is to read the Zed source code or drop a question in the [Zed Discord](https://zed.dev/community-links). We're working on improving the documentation, creating more examples, and will be publishing more guides to GPUI on our [blog](https://zed.dev/blog).
