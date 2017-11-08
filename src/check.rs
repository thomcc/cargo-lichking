use cargo::core::Package;
use cargo::{ human, Config, CargoResult };

use licensed::Licensed;

pub fn run(root: &Package, packages: Vec<Package>, config: &Config) -> CargoResult<()> {
    let mut fail = 0;
    let license = root.license();

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
        Err(human("Incompatible license"))
    } else {
        Ok(())
    }
}
