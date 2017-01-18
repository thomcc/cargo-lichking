extern crate void;
extern crate cargo;
extern crate rustc_serialize;

mod license;
mod licensed;
mod load;
mod check;
mod list;

use cargo::{ Config, CliResult };

const USAGE: &'static str = "
Display info about licensing of dependencies

Usage: cargo lichking (list|check) [options]
       cargo lichking --help

Options:
    -h, --help              Print this message
    -V, --version           Print version info and exit
    -v, --verbose ...       Use verbose output (-vv very verbose output)
    -q, --quiet             Use quiet output
    --manifest-path PATH    Path to the manifest to analyze
    --color WHEN            Coloring: auto, always, never
    --frozen                Require Cargo.lock and cache are up to date
    --locked                Require Cargo.lock is up to date
";

#[derive(RustcDecodable)]
struct Options {
    cmd_list: bool,
    cmd_check: bool,
    flag_version: bool,
    flag_verbose: u32,
    flag_quiet: Option<bool>,
    flag_manifest_path: Option<String>,
    flag_color: Option<String>,
    flag_frozen: bool,
    flag_locked: bool,
}

fn main() {
    cargo::execute_main_without_stdin(real_main, false, USAGE);
}

fn real_main(options: Options, config: &Config) -> CliResult<Option<()>> {
    config.configure(
        options.flag_verbose,
        options.flag_quiet,
        &options.flag_color,
        options.flag_frozen,
        options.flag_locked)?;

    if options.flag_version {
        config.shell().say(format!("cargo-lichking {}", env!("CARGO_PKG_VERSION")), 0)?;
        return Ok(None);
    }

    config.shell().warn("IANAL: This is not legal advice and is not guaranteed to be correct.")?;

    let (root, packages) = load::resolve_packages(options.flag_manifest_path, config)?;

    if options.cmd_check {
        check::run(root, packages, config)?;
    } else if options.cmd_list {
        list::run(packages, config)?;
    } else {
        unreachable!()
    }

    Ok(None)
}
