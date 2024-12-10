#![doc = include_str!("../README.md")]

use core::fmt::{self, Write as _};

use clap_builder::{Command, CommandFactory};
use stackstack::Stack;

/// Create a markdown description for the given [`Command`].
///
/// See [module documentation](mod@self) for more.
pub fn markdown(cmd: Command) -> String {
    let mut buf = String::new();
    _markdown(&mut buf, Stack::new(), &cmd).expect("fmt::write returned an error");
    buf
}

/// Create a markdown description for the given [`CommandFactory`].
///
/// This is shorthand for [`markdown`].
pub fn markdown_for<T: CommandFactory>() -> String {
    markdown(T::command())
}

fn _markdown(buf: &mut String, path: Stack<&str>, cmd: &Command) -> fmt::Result {
    if !path.is_empty() {
        buf.push('\n')
    }
    let path = path.pushed(cmd.get_name());
    for _ in 0..path.len() {
        buf.push('#')
    }
    for component in &path {
        buf.write_fmt(format_args!(" `{component}`"))?;
    }

    let mut cmd = cmd
        .clone()
        .disable_help_subcommand(true)
        .disable_help_flag(true);
    cmd.set_bin_name(join(&path)?); // hack the bin name get a nice `Usage`.
    fmt::write(
        buf,
        format_args!(
            "\n```text\n{}\n```",
            cmd.render_long_help().to_string().trim()
        ),
    )?;
    for sub in cmd.get_subcommands() {
        _markdown(buf, path, sub)?
    }
    Ok(())
}

fn join<T: fmt::Display>(it: impl IntoIterator<Item = T>) -> Result<String, fmt::Error> {
    let mut it = it.into_iter().peekable();
    let mut s = String::new();
    while let Some(comp) = it.next() {
        s.write_fmt(format_args!("{comp}"))?;
        if it.peek().is_some() {
            s.push(' ');
        }
    }
    Ok(s)
}

#[cfg(test)]
mod tests {
    include!("test-snippet.rs");

    use clap::Parser;
    use expect_test::{expect, expect_file};
    use indoc::formatdoc;

    use super::*;

    #[test]
    fn readme() {
        let crate_name = env!("CARGO_PKG_NAME");
        let test_snippet = include_str!("test-snippet.rs").trim();
        let markdown = markdown(Cargo::command());
        let readme = formatdoc! {
            "
            # `{crate_name}`
            Create markdown descriptions for [`clap::Command`]s.

            So given the following rust code:
            ```rust
            {test_snippet}
            ```

            You get the markdown that follows,
            with subcommands handled as you'd expect.
            ---
            {markdown}
            "
        };
        expect_file!["../README.md"].assert_eq(&readme);
    }

    /// This is a top-level description.
    #[derive(Parser)]
    #[command(name = "simple")]
    struct Simple {
        /// This is a flag.
        #[arg(short, long)]
        flag: bool,

        /// This is a mandatory positional argument.
        pos: String,
        /// This is a mandatory switched argument.
        #[arg(short, long)]
        switch: String,

        /// This is an optional positional argument.
        opt_pos: Option<String>,
        /// This is an optional switched argument.
        #[arg(short, long)]
        opt_switch: Option<String>,

        /// This is a switched argument with a default
        #[arg(short, long, default_value = "default")]
        default_switch: String,
    }

    #[test]
    fn simple() {
        expect![[r#"
            # `simple`
            ```text
            This is a top-level description

            Usage: simple [OPTIONS] --switch <SWITCH> <POS> [OPT_POS]

            Arguments:
              <POS>
                      This is a mandatory positional argument

              [OPT_POS]
                      This is an optional positional argument

            Options:
              -f, --flag
                      This is a flag

              -s, --switch <SWITCH>
                      This is a mandatory switched argument

              -o, --opt-switch <OPT_SWITCH>
                      This is an optional switched argument

              -d, --default-switch <DEFAULT_SWITCH>
                      This is a switched argument with a default
                      
                      [default: default]
            ```"#]]
        .assert_eq(&markdown(Simple::command()));
    }
}
