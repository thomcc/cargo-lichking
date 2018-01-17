#![allow(unknown_lints)] // for clippy

#[macro_use] extern crate clap;
extern crate cargo;
extern crate void;

mod license;
mod licensed;
mod load;
mod check;
mod list;
mod options;

use std::process;

use cargo::{ Config, CliResult };

use options::{ Options, Cmd };

fn main() {
    let matches = Options::app(false).get_matches();
    let options = Options::from_matches(&matches);
    let mut config = Config::default().expect("No idea why this would fail");
    let result = real_main(options, &mut config);
    if let Err(err) = result {
        config.shell().error(err).expect("Can't do much");
        process::exit(1);
    }
}

fn real_main(options: Options, config: &mut Config) -> CliResult {
    config.configure(
        options.verbose,
        Some(options.quiet),
        &options.color,
        options.frozen,
        options.locked,
        &[])?;

    config.shell().warn("IANAL: This is not legal advice and is not guaranteed to be correct.")?;

    let manifest_path = options.manifest_path;

    match options.cmd {
        Cmd::Check { package } => {
            let mut error = Ok(());
            let roots = load::resolve_roots(manifest_path.clone(), config, package)?;
            for root in roots {
                let packages = load::resolve_packages(manifest_path.clone(), config, vec![&root])?;
                if let Err(err) = check::run(&root, packages, config) {
                    error = Err(err);
                }
            }
            error?;
        }
        Cmd::List { by, package } => {
            let roots = load::resolve_roots(manifest_path.clone(), config, package)?;
            let packages = load::resolve_packages(manifest_path, config, &roots)?;
            list::run(packages, by)?;
        }
    }

    Ok(())
}
