use std::fmt;
use std::str::FromStr;
use std::path::PathBuf;
use void::Void;

#[derive(Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
#[allow(non_camel_case_types)]
pub enum License {
  MIT,
  X11,
  BSD_3_Clause,
  Apache_2_0,
  LGPL_2_0,
  LGPL_2_1Plus,
  LGPL_3_0,
  MPL_1_1,
  GPL_2_0,
  GPL_2_0Plus,
  GPL_3_0,
  GPL_3_0Plus,
  AGPL_1_0,
  Custom(String),
  File(PathBuf),
  Multiple(Vec<License>),
  None,
}

impl Default for License {
  fn default() -> License {
    License::None
  }
}

impl FromStr for License {
  type Err = Void;
  fn from_str(s: &str) -> Result<License, Void> {
    match s {
      "MIT"                => Ok(License::MIT),
      "X11"                => Ok(License::X11),
      "BSD-3-Clause"       => Ok(License::BSD_3_Clause),
      "Apache-2.0"         => Ok(License::Apache_2_0),
      "LGPL-2.0"           => Ok(License::LGPL_2_0),
      "LGPL-2.1+"          => Ok(License::LGPL_2_1Plus),
      "LGPL-3.0"           => Ok(License::LGPL_3_0),
      "MPL-1.1"            => Ok(License::MPL_1_1),
      "GPL-2.0"            => Ok(License::GPL_2_0),
      "GPL-2.0+"           => Ok(License::GPL_2_0Plus),
      "GPL-3.0"            => Ok(License::GPL_3_0),
      "GPL-3.0+"           => Ok(License::GPL_3_0Plus),
      "AGPL-1.0"           => Ok(License::AGPL_1_0),
      s if s.contains('/') => {
        let mut licenses: Vec<License> = s.split('/').map(str::parse).map(Result::unwrap).collect();
        licenses.sort();
        Ok(License::Multiple(licenses))
      },
      s                    => Ok(License::Custom(s.to_owned())),
    }
  }
}

impl fmt::Display for License {
  fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      License::MIT           => w.write_str("MIT"),
      License::X11           => w.write_str("X11"),
      License::BSD_3_Clause  => w.write_str("BSD-3-Clause"),
      License::Apache_2_0    => w.write_str("Apache-2.0"),
      License::LGPL_2_0      => w.write_str("LGPL-2.0"),
      License::LGPL_2_1Plus  => w.write_str("LGPL-2.1+"),
      License::LGPL_3_0      => w.write_str("LGPL-3.0"),
      License::MPL_1_1       => w.write_str("MPL-1.1"),
      License::GPL_2_0       => w.write_str("GPL-2.0"),
      License::GPL_2_0Plus   => w.write_str("GPL-2.0+"),
      License::GPL_3_0       => w.write_str("GPL-3.0"),
      License::GPL_3_0Plus   => w.write_str("GPL-3.0+"),
      License::AGPL_1_0      => w.write_str("AGPL-1.0"),
      License::Custom(ref s) => write!(w, "Custom ({})", s),
      License::File(ref f)   => write!(w, "File ({})", f.to_string_lossy()),
      License::Multiple(ref ls)   => {
        try!(w.write_str("Any of "));
        try!(fmt::Display::fmt(&ls[0], w));
        for l in ls.iter().skip(1) {
          try!(w.write_str(", "));
          try!(fmt::Display::fmt(l, w));
        }
        Ok(())
      },
      License::None          => w.write_str("Unlicensed"),
    }
  }
}

