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
