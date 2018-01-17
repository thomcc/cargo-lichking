use std::io;
use std::fs::File;

use cargo::{Config, CargoResult};
use cargo::core::{Package, Shell};

use license::License;
use licensed::Licensed;
use options::Bundle;
use discovery::{
    Confidence, LicenseText, find_generic_license_text, find_license_text
};

struct Context<'a, 'b> {
    root: Package,
    packages: &'a [Package],
    shell: &'b mut Shell,

    missing_license: bool,
    low_quality_license: bool,
}

pub fn run(root: Package, mut packages: Vec<Package>, config: &Config, variant: Bundle) -> CargoResult<()> {
    packages.sort_by_key(|package| package.name().to_owned());

    let mut context = Context {
        root: root,
        packages: &packages,
        shell: &mut config.shell(),
        missing_license: false,
        low_quality_license: false,
    };

    match variant {
        Bundle::Inline { file } => {
            if let Some(file) = file {
                inline(&mut context, &mut File::open(file)?)?;
            } else {
                inline(&mut context, &mut io::stdout())?;
            }
        }
    }

    if context.missing_license {
        context.shell.error("
  Our liches failed to recognise a license in one or more packages.

  We would be very grateful if you could check the corresponding package
  directories (see the package specific message above) to see if there is an
  easily recognisable license file available.

  If there is please submit details to
      https://github.com/Nemo157/cargo-lichking/issues
  so we can make sure this license is recognised in the future.

  If there isn't you could submit an issue to the package's project asking
  them to include the text of their license in the built packages.")?;
    }

    if context.low_quality_license {
        context.shell.error("\
  Our liches are very unsure about one or more licenses that were put into the \
  bundle. Please check the specific error messages above.")?;
    }

    if context.missing_license || context.low_quality_license {
        Err("Generating bundle finished with error(s)")?
    } else {
        Ok(())
    }
}

fn inline(context: &mut Context, mut out: &mut io::Write) -> CargoResult<()> {
    writeln!(out, "The {} package uses some third party libraries under their own license terms:", context.root.name())?;
    writeln!(out, "")?;
    for package in context.packages {
        inline_package(context, package, out)?;
        writeln!(out, "")?;
    }
    Ok(())
}

fn inline_package(context: &mut Context, package: &Package, mut out: &mut io::Write) -> CargoResult<()> {
    let license = package.license();
    writeln!(out, " * {} under {}:", package.name(), license)?;
    writeln!(out, "")?;
    if let Some(text) = find_generic_license_text(package, &license)? {
        match text.confidence {
            Confidence::Confident => (),
            Confidence::SemiConfident => {
                context.shell.warn(format_args!("{} has only a low-confidence candidate for license {}:", package.name(), license))?;
                context.shell.warn(format_args!("    {}", text.path.display()))?;
            }
            Confidence::Unsure => {
                context.shell.error(format_args!("{} has only a very low-confidence candidate for license {}:", package.name(), license))?;
                context.shell.error(format_args!("    {}", text.path.display()))?;
            }
        }
        for line in text.text.lines() {
            writeln!(out, "    {}", line)?;
        }
    } else {
        match license {
            License::Unspecified => {
                context.shell.error(format_args!("{} does not specify a license", package.name()))?;
            }
            License::Multiple(licenses) => {
                let mut first = true;
                for license in licenses {
                    if first {
                        first = false;
                    } else {
                        writeln!(out, "")?;
                        writeln!(out, "    ===============")?;
                        writeln!(out, "")?;
                    }
                    inline_license(context, package, &license, out)?;
                }
            }
            license => {
                inline_license(context, package, &license, out)?;
            }
        }
    }
    writeln!(out, "")?;
    Ok(())
}

fn inline_license(context: &mut Context, package: &Package, license: &License, mut out: &mut io::Write) -> CargoResult<()> {
    let texts = find_license_text(package, license)?;
    if let Some(text) = choose(context, package, license, texts)? {
        for line in text.text.lines() {
            writeln!(out, "    {}", line)?;
        }
    }
    Ok(())
}

fn choose(context: &mut Context, package: &Package, license: &License, texts: Vec<LicenseText>) -> CargoResult<Option<LicenseText>> {
    let (mut confident, texts): (Vec<LicenseText>, Vec<LicenseText>) = texts.into_iter().partition(|text| text.confidence == Confidence::Confident);
    let (mut semi_confident, mut unconfident): (Vec<LicenseText>, Vec<LicenseText>) = texts.into_iter().partition(|text| text.confidence == Confidence::SemiConfident);

    if confident.len() == 1 {
        return Ok(Some(confident.swap_remove(0)));
    } else if confident.len() > 1 {
        context.shell.error(format_args!("{} has multiple candidates for license {}:", package.name(), license))?;
        for text in &confident {
            context.shell.error(format_args!("    {}", text.path.display()))?;
        }
        return Ok(Some(confident.swap_remove(0)));
    }

    if semi_confident.len() == 1 {
        context.shell.warn(format_args!("{} has only a low-confidence candidate for license {}:", package.name(), license))?;
        context.shell.warn(format_args!("    {}", semi_confident[0].path.display()))?;
        return Ok(Some(semi_confident.swap_remove(0)));
    } else if semi_confident.len() > 1 {
        context.low_quality_license = true;
        context.shell.error(format_args!("{} has multiple low-confidence candidates for license {}:", package.name(), license))?;
        for text in &semi_confident {
            context.shell.error(format_args!("    {}", text.path.display()))?;
        }
        return Ok(Some(semi_confident.swap_remove(0)));
    }

    if unconfident.len() == 1 {
        context.low_quality_license = true;
        context.shell.warn(format_args!("{} has only a very low-confidence candidate for license {}:", package.name(), license))?;
        context.shell.warn(format_args!("    {}", unconfident[0].path.display()))?;
        return Ok(Some(unconfident.swap_remove(0)));
    } else if unconfident.len() > 1 {
        context.low_quality_license = true;
        context.shell.error(format_args!("{} has multiple very low-confidence candidates for license {}:", package.name(), license))?;
        for text in &unconfident {
            context.shell.error(format_args!("    {}", text.path.display()))?;
        }
        return Ok(Some(unconfident.swap_remove(0)));
    }

    context.shell.error(format_args!("{} has no candidate texts for license {} in {}", package.name(), license, package.root().display()))?;
    context.missing_license = true;
    return Ok(None);
}
