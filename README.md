# `clap-doc`
Create markdown descriptions for [`clap::Command`]s.

So given the following rust code:
```rust
/// Rust's package manager
#[derive(clap::Parser)]
#[command(name = "cargo")]
enum Cargo {
    /// Compile a local package and all of its dependencies
    Build {
        /// Build artifacts in release mode, with optimizations
        #[arg(short, long)]
        release: bool,
    },
    /// Run a binary or example of the local package
    Run {
        /// Arguments for the binary or example to run
        args: Vec<String>,
    },
    /// Remove artifacts that cargo has generated in the past
    Clean,
}
```

You get the markdown that follows,
with subcommands handled as you'd expect.
---
# `cargo`
```text
Rust's package manager

Usage: cargo <COMMAND>

Commands:
  build  Compile a local package and all of its dependencies
  run    Run a binary or example of the local package
  clean  Remove artifacts that cargo has generated in the past
```
## `cargo` `build`
```text
Compile a local package and all of its dependencies

Usage: cargo build [OPTIONS]

Options:
  -r, --release
          Build artifacts in release mode, with optimizations
```
## `cargo` `run`
```text
Run a binary or example of the local package

Usage: cargo run [ARGS]...

Arguments:
  [ARGS]...
          Arguments for the binary or example to run
```
## `cargo` `clean`
```text
Remove artifacts that cargo has generated in the past

Usage: cargo clean
```
