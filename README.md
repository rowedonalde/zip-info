# zip-info

```
Usage: zi [-j | -p] [--exclude=<glob>] <path> ...
       zi --help
zi presents information about Zip archives.
Common options:
    -h, --help         Show this usage message.
    -j, --json         Structure the output in JSON
    -p, --pretty-json  Structure the output in easy-to-read JSON
    --exclude=<glob>   Ignore objects in the archives whose name
                       is like this glob pattern.
```

## How to set up your development environment
1. Make sure you are on nightly. One of the crates we use in this project uses a feature flag and will not compile on stable. The easiest way to do this is `rustup install nightly`.
2. `rustup default nightly` and then `cargo build`
3. You should be able to run `targets/debug/zi` now.
4. (**Bonus**) You can set up VS Code with the Rust Language Server and GDB for a full IDE experience. See the following section for detailed instructions.

### Rust Language Server for an IDE experience using Visual Studio Code
  - You will need to be on nightly to install the Rust Language Server (RLS). Specific directions for this can be found [here](https://github.com/rust-lang-nursery/rls).
  - You will need to install [this VS Code extension](https://github.com/editor-rs/vscode-rust).
  - I had to add the following configuration in my user settings to help the VS Code extension find my RLS installation. This worked on my Fedora 26 machine, but your mileage will vary depending on how your `rustup` and operating system are set up.
    ```
    "rust.rls": {
            "executable": "rustup",
            "args": ["run", "nightly", "rls"]
        }
    ```
   - You should see a status indicator in the bottom left-hand corner of VS Code with something along the lines of `RLS: Analysis finished`.

### Setting up GDB debugging with the VS Code extension
  - Since OS X ditched gdb for LLDB, you will need to install GDB if you want to use this VS Code extension on a Mac. ([Source](https://medium.com/@royalstream/rust-development-using-vs-code-on-os-x-debugging-included-bc10c9863777))
  - Install [this plugin](https://github.com/WebFreak001/code-debug). (The gdb target settings in `launch.json` are already defined for you inside the `.vscode` folder, so you should see a green play arrow with the target name **Debug** already ready for you when you open the Debugging pane in VS Code.)
  - There are Rust GDB pretty-printing scripts that are already loaded in the `etc` folder of this repository, straight from the Rust distribution, since we cannot call `rust-gdb` from our VS Code extension (and Windows installations will automatically omit these files, so we will have to end up manually copying them over anyway to debug on Windows). These pretty-printers will be loaded by the `.gdbinit` in the root directory of this repository.
  - On my Fedora machine, I had a problem with this project's `.gdbinit` not loading properly until it was whitelisted in the `.gdbinit` in my home directory. In that home directory `.gdbinit`, I had to add this following bit to whitelist this project's `.gdbinit`:
    ```
    add-auto-load-safe-path <PATH_TO_THIS_REPO_FOLDER>
    ```
    (You can also whitelist all `.gdbinit` on your system by adding the bit `add-auto-load-safe-path /`.)
