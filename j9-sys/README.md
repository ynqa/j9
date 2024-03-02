# j9-sys

j9-sys is a Rust crate that provides bindings to
[libjq](https://github.com/jqlang/jq),
the library behind jq, a lightweight and flexible command-line JSON processor.
It allows Rust developers to integrate jq's JSON processing features directly
into their applications.

## jq Version

This crate is currently using jq version 1.7.1.

## Building

To build this crate, `clang` is required.
The reason for this is that the crate uses `bindgen`
for generating the Rust bindings to the native `libjq` library.
`bindgen` itself relies on `clang` to parse the header files of the C library.

If you do not have `clang` or encounter any issues with your local environment,
you can use the provided [.devcontainer](../.devcontainer) setup.
This setup includes a pre-configured environment
with Rust and `clang` installed, ensuring a smooth build process.

To use the `.devcontainer`, you need to have Docker installed
and `Remote - Containers` extension for Visual Studio Code.
Once set up, simply open the project in VS Code and choose to reopen it in a container
when prompted. This will automatically set up an environment
where you can build and test the crate without needing
to manually install the dependencies on your system.

To build this crate, use the following command:

```bash
$ cargo build
```
