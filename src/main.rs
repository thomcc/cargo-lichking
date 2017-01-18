extern crate void;
extern crate cargo;
extern crate rustc_serialize;

mod license;
mod licensed;

use std::collections::{ HashMap, HashSet };

use cargo::core::dependency::Kind;
use cargo::core::registry::PackageRegistry;
use cargo::core::resolver::Resolve;
use cargo::core::{ Package, PackageId, Workspace };
use cargo::ops;
use cargo::util::CargoResult;
use cargo::util::important_paths::find_root_manifest_for_wd;
use cargo::{ human, Config, CliResult };

use licensed::Licensed;

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

    let root = find_root_manifest_for_wd(options.flag_manifest_path, config.cwd())?;
    let workspace = Workspace::new(&root, config)?;
    let current = workspace.current()?;
    let mut registry = PackageRegistry::new(config)?;
    registry.add_sources(&[current.package_id().source_id().clone()])?;
    let resolve = ops::resolve_ws(&mut registry, &workspace)?;
    let packages = get_packages(current.package_id(), &resolve, registry)?;

    if options.cmd_check {
        let mut fail = 0;
        let license = current.license();
        for package in packages {
            let can_include = license.can_include(&package.license());
            if let Some(can_include) = can_include {
                if !can_include {
                    config.shell().error(format!("Cannot include package {}, license {} is incompatible with {}", package.name(), package.license(), license))?;
                    fail += 1;
                }
            } else {
                config.shell().warn(format!("Unknown whether package {} with license {} is compatible with {}", package.name(), package.license(), license))?;
            }
        }
        if fail > 0 {
            Err(human("Incompatible license").into())
        } else {
            Ok(None)
        }
    } else if options.cmd_list {
        let mut license_to_packages = HashMap::new();

        for package in packages {
            let list = license_to_packages.entry(package.license()).or_insert(Vec::new());
            list.push(package);
        }

        for (license, packages) in license_to_packages {
            config.shell().say(format!("{}: {}", license, packages.iter().map(|package| package.name()).collect::<Vec<&str>>().join(", ")), 0)?;
        }

        Ok(None)
    } else {
        unreachable!()
    }
}

fn get_packages(root: &PackageId, resolve: &Resolve, registry: PackageRegistry) -> CargoResult<Vec<Package>> {
    let packages = ops::get_resolved_packages(resolve, registry);

    let mut result = HashSet::new();
    let mut to_check = vec![root];
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
