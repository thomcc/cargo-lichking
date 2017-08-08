use std::collections::HashMap;

use cargo::core::Package;
use cargo::{ Config, CargoResult };

use licensed::Licensed;
use options::By;

pub fn run(mut packages: Vec<Package>, config: &Config, by: By) -> CargoResult<()> {
    match by {
        By::License => {
            let mut license_to_packages = HashMap::new();

            for package in packages {
                license_to_packages
                    .entry(package.license())
                    .or_insert_with(Vec::new)
                    .push(package);
            }

            let mut license_to_packages = license_to_packages.iter().collect::<Vec<_>>();
            license_to_packages.sort_by_key(|&(license, _)| license);

            for (license, packages) in license_to_packages {
                let packages = packages.iter().map(|package| package.name()).collect::<Vec<&str>>().join(", ");
                config.shell().say(format!("{}: {}", license, packages), 0)?;
            }
        }
        By::Crate => {
            packages.sort_by_key(|package| package.name().to_owned());
            for package in packages {
                config.shell().say(format!("{}: {}", package.name(), package.license()), 0)?;
            }
        }
    }

    Ok(())
}
