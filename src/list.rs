use std::collections::HashMap;

use cargo::core::Package;
use cargo::{ Config, CargoResult };

use licensed::Licensed;

pub fn run(packages: Vec<Package>, config: &Config) -> CargoResult<()> {
    let mut license_to_packages = HashMap::new();

    for package in packages {
        license_to_packages
            .entry(package.license())
            .or_insert(Vec::new())
            .push(package);
    }

    for (license, packages) in license_to_packages {
        let packages = packages.iter().map(|package| package.name()).collect::<Vec<&str>>().join(", ");
        config.shell().say(format!("{}: {}", license, packages), 0)?;
    }

    Ok(())
}
