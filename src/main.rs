extern crate void;
extern crate cargo;
extern crate rustc_serialize;

mod license;
mod licensed;

use std::collections::{ HashMap, HashSet };

use cargo::core::dependency::Kind;
use cargo::core::registry::PackageRegistry;
use cargo::core::resolver::Resolve;
use cargo::core::{ Source, Package };
use cargo::core::source::SourceId;
use cargo::ops;
use cargo::sources::path::PathSource;
use cargo::util::{ important_paths, CargoResult };
use cargo::{ Config, CliResult, CliError };

use licensed::Licensed;

const USAGE: &'static str = "
Display info about licensing of dependencies

Usage: cargo lichking (list|check) [options]
       cargo lichking --help

Options:
    -h, --help              Print this message
    -V, --version           Print version info and exit
    -v, --verbose           Use verbose output
    -q, --quiet             Use quiet output
    --manifest-path PATH    Path to the manifest to analyze
";

#[derive(RustcDecodable)]
struct Flags {
  cmd_list: bool,
  cmd_check: bool,
  flag_version: bool,
  flag_verbose: bool,
  flag_quiet: bool,
  flag_manifest_path: Option<String>,
}

fn main() {
  cargo::execute_main_without_stdin(real_main, false, USAGE);
}

fn real_main(flags: Flags, config: &Config) -> CliResult<Option<()>> {
  let Flags {
    cmd_list,
    cmd_check,
    flag_version,
    flag_verbose,
    flag_quiet,
    flag_manifest_path,
  } = flags;

  if flag_version {
    println!("cargo-lichking {}", env!("CARGO_PKG_VERSION"));
    return Ok(None);
  }

  println!("IANAL: This is not legal advice and is not guaranteed to be correct.");

  try!(config.configure_shell(Some(flag_verbose), Some(flag_quiet), &None));

  let mut source = try!(source(config, flag_manifest_path));
  let root = try!(source.root_package());
  let mut registry = try!(registry(config, &root));
  let resolve = try!(ops::resolve_pkg(&mut registry, &root, config));
  let packages = try!(get_packages(&resolve, registry));

  if cmd_check {
    let mut fail = 0;
    let license = root.license();
    for package in packages {
      let can_include = license.can_include(&package.license());
      if let Some(can_include) = can_include {
        if !can_include {
          println!("Error: Cannot include package {}, license {} is incompatible with {}", package.name(), package.license(), license);
          fail += 1;
        }
      } else {
        println!("Warning: Unknown whether package {} with license {} is compatible with {}", package.name(), package.license(), license);
      }
    }
    if fail > 0 {
      Err(CliError::new("Incompatible license", fail))
    } else {
      Ok(None)
    }
  } else if cmd_list {
    let mut license_to_packages = HashMap::new();

    for package in packages {
      let list = license_to_packages.entry(package.license()).or_insert(Vec::new());
      list.push(package);
    }

    for (license, packages) in license_to_packages {
      println!("{}: {}", license, packages.iter().map(|package| package.name()).collect::<Vec<&str>>().join(", "));
    }

    Ok(None)
  } else {
    unreachable!()
  }
}

fn get_packages(resolve: &Resolve, registry: PackageRegistry) -> CargoResult<Vec<Package>> {
  let packages = ops::get_resolved_packages(resolve, registry);

  let mut result = HashSet::new();
  let mut to_check = vec![resolve.root()];
  while let Some(id) = to_check.pop() {
    if let Ok(package) = packages.get(id) {
      if result.insert(package) {
        let deps = resolve.deps(id);
        for dep_id in deps {
          let dep = package.dependencies().iter().find(|d| d.matches_id(dep_id)).unwrap();
          if let Kind::Normal = dep.kind() {
            to_check.push(dep_id);
          }
        }
      }
    }
  }

  Ok(result.into_iter().cloned().collect())
}

fn source(config: &Config, manifest_path: Option<String>) -> CargoResult<PathSource> {
  let root = try!(important_paths::find_root_manifest_for_wd(manifest_path, config.cwd()));
  let parent = root.parent().unwrap();
  let mut source = PathSource::new(parent, &try!(SourceId::for_path(parent)), config);
  try!(source.update());
  Ok(source)
}

fn registry<'a>(config: &'a Config, package: &Package) -> CargoResult<PackageRegistry<'a>> {
  let mut registry = PackageRegistry::new(config);
  try!(registry.add_sources(&[package.package_id().source_id().clone()]));
  Ok(registry)
}
