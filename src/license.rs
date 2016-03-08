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
  LGPL_2_1,
  LGPL_2_1Plus,
  LGPL_3_0,
  LGPL_3_0Plus,
  MPL_1_1,
  MPL_2_0,
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

macro_rules! compatibility {
  ($s:expr, $o:expr, { $($a:pat => $($b:pat),+;)+ }) => {
    match $s {
      $(
        $a => match $o {
          $($b)|+ => return Some(true),
          _ => (),
        }
      ),*
    }
  };
}

impl License {
  pub fn can_include(&self, other: &License) -> Option<bool> {
    use self::License::*;

    if let None = *other { return Some(false); }

    if let Custom(_) = *self { return Option::None; }
    if let Custom(_) = *other { return Option::None; }
    if let File(_) = *self { return Option::None; }
    if let File(_) = *other { return Option::None; }

    if let Multiple(ref licenses) = *self {
      for license in licenses {
        if let Some(can_include) = license.can_include(other) {
          if !can_include {
            return Some(false);
          }
        } else {
          return Option::None;
        }
      }
      return Some(true);
    }

    if let Multiple(ref licenses) = *other {
      let mut seen_none = false;
      for license in licenses {
        if let Some(can_include) = self.can_include(license) {
          if can_include {
            return Some(true);
          }
        } else {
          seen_none = true;
        }
      }
      if seen_none {
        return Option::None;
      } else {
        return Some(false);
      }
    }

    if let LGPL_2_0 = *self { return Option::None; /* TODO: unknown */ }
    if let LGPL_2_0 = *other { return Option::None; /* TODO: unknown */ }

    compatibility!(*self, *other, {
      None         => MIT, X11, BSD_3_Clause;

      LGPL_2_0     => LGPL_2_0; // TODO: probably allows more

      MIT          => MIT, X11;
      X11          => MIT, X11;
      BSD_3_Clause => MIT, X11, BSD_3_Clause;
      Apache_2_0   => MIT, X11, BSD_3_Clause, Apache_2_0;
      MPL_1_1      => MIT, X11, BSD_3_Clause, MPL_1_1;
      MPL_2_0      => MIT, X11, BSD_3_Clause, MPL_2_0;
      LGPL_2_1Plus => MIT, X11, BSD_3_Clause, MPL_2_0, LGPL_2_1Plus;
      LGPL_2_1     => MIT, X11, BSD_3_Clause, MPL_2_0, LGPL_2_1Plus, LGPL_2_1;
      LGPL_3_0Plus => MIT, X11, BSD_3_Clause, MPL_2_0, Apache_2_0, LGPL_2_1Plus, LGPL_3_0Plus;
      LGPL_3_0     => MIT, X11, BSD_3_Clause, MPL_2_0, Apache_2_0, LGPL_2_1Plus, LGPL_3_0Plus, LGPL_3_0;
      GPL_2_0Plus  => MIT, X11, BSD_3_Clause, MPL_2_0, LGPL_2_1Plus, LGPL_2_1, GPL_2_0Plus;
      GPL_2_0      => MIT, X11, BSD_3_Clause, MPL_2_0, LGPL_2_1Plus, LGPL_2_1, GPL_2_0Plus, GPL_2_0;
      GPL_3_0Plus  => MIT, X11, BSD_3_Clause, MPL_2_0, Apache_2_0, LGPL_2_1Plus, LGPL_2_1, GPL_2_0Plus, GPL_3_0Plus;
      GPL_3_0      => MIT, X11, BSD_3_Clause, MPL_2_0, Apache_2_0, LGPL_2_1Plus, LGPL_2_1, GPL_2_0Plus, GPL_3_0Plus, GPL_3_0;
      AGPL_1_0     => MIT, X11, BSD_3_Clause, MPL_2_0, Apache_2_0, LGPL_2_1Plus, LGPL_2_1, GPL_2_0Plus, GPL_3_0Plus, GPL_3_0, AGPL_1_0;

      // TODO: These are `unreachable!()`, can't figure out a nice way to allow this in the macro...
      Custom(_)    => MIT;
      File(_)      => MIT;
      Multiple(_)  => MIT;
    });

    return Some(false);
  }
}

impl FromStr for License {
  type Err = Void;
  fn from_str(s: &str) -> Result<License, Void> {
    match s.trim() {
      "MIT"                => Ok(License::MIT),
      "X11"                => Ok(License::X11),
      "BSD-3-Clause"       => Ok(License::BSD_3_Clause),
      "Apache-2.0"         => Ok(License::Apache_2_0),
      "LGPL-2.0"           => Ok(License::LGPL_2_0),
      "LGPL-2.1"           => Ok(License::LGPL_2_1),
      "LGPL-2.1+"          => Ok(License::LGPL_2_1Plus),
      "LGPL-3.0"           => Ok(License::LGPL_3_0),
      "LGPL-3.0+"          => Ok(License::LGPL_3_0Plus),
      "MPL-1.1"            => Ok(License::MPL_1_1),
      "MPL-2.0"            => Ok(License::MPL_2_0),
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
      License::LGPL_2_1      => w.write_str("LGPL-2.1"),
      License::LGPL_2_1Plus  => w.write_str("LGPL-2.1+"),
      License::LGPL_3_0      => w.write_str("LGPL-3.0"),
      License::LGPL_3_0Plus  => w.write_str("LGPL-3.0+"),
      License::MPL_1_1       => w.write_str("MPL-1.1"),
      License::MPL_2_0       => w.write_str("MPL-2.0"),
      License::GPL_2_0       => w.write_str("GPL-2.0"),
      License::GPL_2_0Plus   => w.write_str("GPL-2.0+"),
      License::GPL_3_0       => w.write_str("GPL-3.0"),
      License::GPL_3_0Plus   => w.write_str("GPL-3.0+"),
      License::AGPL_1_0      => w.write_str("AGPL-1.0"),
      License::Custom(ref s) => write!(w, "Custom({})", s),
      License::File(ref f)   => write!(w, "File({})", f.to_string_lossy()),
      License::Multiple(ref ls)   => {
        try!(w.write_str("Any("));
        try!(fmt::Display::fmt(&ls[0], w));
        for l in ls.iter().skip(1) {
          try!(w.write_str(", "));
          try!(fmt::Display::fmt(l, w));
        }
        w.write_str(")")
      },
      License::None          => w.write_str("Unlicensed"),
    }
  }
}

