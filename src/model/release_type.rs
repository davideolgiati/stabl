use std::str::FromStr;
use std::fmt::{self, Display, Formatter};

#[derive(Clone)]
#[derive(PartialEq, PartialOrd)]
pub enum ReleaseType {
        Repack,
        Patch,
        Minor,
        Major
}

impl FromStr for ReleaseType {
        type Err = String;
    
        fn from_str(input: &str) -> Result<ReleaseType, Self::Err> {
                match input.to_lowercase().as_str() {
                        "security"      => Ok(ReleaseType::Patch),
                        "bugfix"        => Ok(ReleaseType::Patch),
                        "enhancement"   => Ok(ReleaseType::Minor),
                        "unspecified"   => Ok(ReleaseType::Major),
                        _               => Err(format!("'{}' is not a valid value for ReleaseType", input)),
                }
        }
}

impl Display for ReleaseType {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            match self {
                Self::Major  => write!(f, "major"),
                Self::Minor  => write!(f, "minor"),
                Self::Patch  => write!(f, "patch"),
                Self::Repack => write!(f, "repack"),
            }
        }
    }
    