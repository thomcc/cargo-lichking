extern crate void;
extern crate cargo;
extern crate rustc_serialize;

mod license;
mod licensed;

use std::collections::HashMap;
use cargo::core::registry::PackageRegistry;
use cargo::core::{ Source, Package };
use cargo::ops;
use cargo::sources::path::PathSource;
use cargo::util::{ important_paths, CargoResult };
use cargo::{ Config, CliResult };
use licensed::Licensed;

const USAGE: &'static str = "
Display info about licensing of dependencies

Usage: cargo-lichking [options]
       cargo-lichking --help

Options:
    -h, --help              Print this message
    -V, --version           Print version info and exit
    -v, --verbose           Use verbose output
    -q, --quiet             Use quiet output
";

#[derive(RustcDecodable)]
struct Flags {
  flag_version: bool,
  flag_verbose: bool,
  flag_quiet: bool,
}

fn main() {
  cargo::execute_main_without_stdin(real_main, false, USAGE);
}

fn real_main(flags: Flags, config: &Config) -> CliResult<Option<()>> {
  let Flags {
    flag_version,
    flag_verbose,
    flag_quiet,
  } = flags;

  if flag_version {
    println!("cargo-lichking {}", env!("CARGO_PKG_VERSION"));
    return Ok(None);
  }

  try!(config.shell().set_verbosity(flag_verbose, flag_quiet));

  let mut source = try!(source(config));
  let package = try!(source.root_package());
  let mut registry = try!(registry(config, &package));
  let resolve = try!(ops::resolve_pkg(&mut registry, &package));
  let packages = try!(ops::get_resolved_packages(&resolve, &mut registry));

  let mut license_to_packages = HashMap::new();

  for package in packages {
    let list = license_to_packages.entry(package.license()).or_insert(Vec::new());
    list.push(package);
  }

  for (license, packages) in license_to_packages {
    println!("{}: {}", license, packages.iter().map(|package| package.name()).collect::<Vec<&str>>().join(", "));
  }

  Ok(None)
}

fn source(config: &Config) -> CargoResult<PathSource> {
  let root = try!(important_paths::find_root_manifest_for_wd(None, config.cwd()));
  let mut source = try!(PathSource::for_path(root.parent().unwrap(), config));
  try!(source.update());
  Ok(source)
}

fn registry<'a>(config: &'a Config, package: &Package) -> CargoResult<PackageRegistry<'a>> {
  let mut registry = PackageRegistry::new(config);
  try!(registry.add_sources(&[package.package_id().source_id().clone()]));
  Ok(registry)
}
