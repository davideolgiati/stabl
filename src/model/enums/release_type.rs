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

pub fn get_super(value: &ReleaseType) -> ReleaseType {
    match *value {
        ReleaseType::Major => ReleaseType::Major,
        ReleaseType::Minor => ReleaseType::Major,
        ReleaseType::Patch => ReleaseType::Minor,
        ReleaseType::Repack => ReleaseType::Patch
    }
}

impl FromStr for ReleaseType {
    type Err = String;

    fn from_str(input: &str) -> Result<ReleaseType, Self::Err> {
        assert!(!input.is_empty());
    
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
            Self::Major  => write!(f, "MAJOR"),
            Self::Minor  => write!(f, "MINOR"),
            Self::Patch  => write!(f, "PATCH"),
            Self::Repack => write!(f, "REPACK"),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn new_major() {
        let input: &str = "unspecified";
        let expected: ReleaseType = ReleaseType::Major;
        let output = ReleaseType::from_str(input).unwrap();

        assert!(output == expected);
    }

    #[test]
    fn new_minor() {
        let input: &str = "enhancement";
        let expected: ReleaseType = ReleaseType::Minor;
        let output = ReleaseType::from_str(input).unwrap();

        assert!(output == expected);
    }

    #[test]
    fn new_patch() {
        let input: &str = "bugfix";
        let expected: ReleaseType = ReleaseType::Patch;
        let output = ReleaseType::from_str(input).unwrap();

        assert!(output == expected);
    }

    #[test]
    fn new_patch_security() {
        let input: &str = "security";
        let expected: ReleaseType = ReleaseType::Patch;
        let output = ReleaseType::from_str(input).unwrap();

        assert!(output == expected);
    }

    #[test]
    fn print_major() {
        let input: &str = "unspecified";
        let expected: &str = "MAJOR";
        let output = ReleaseType::from_str(input).unwrap();

        assert!(format!("{}", output) == expected);
    }

    #[test]
    fn print_minor() {
        let input: &str = "enhancement";
        let expected: &str = "MINOR";
        let output = ReleaseType::from_str(input).unwrap();

        assert!(format!("{}", output) == expected);
    }

    #[test]
    fn print_patch() {
        let input: &str = "bugfix";
        let expected: &str = "PATCH";
        let output = ReleaseType::from_str(input).unwrap();

        assert!(format!("{}", output) == expected);
    }

    #[test]
    fn print_patch_security() {
        let input: &str = "security";
        let expected: &str = "PATCH";
        let output = ReleaseType::from_str(input).unwrap();

        assert!(format!("{}", output) == expected);
    }

    #[test]
    fn unknown_string() {
        let input: &str = "major";
        let output = ReleaseType::from_str(input);

        assert!(output.is_err(), "'major' is not a valid value for ReleaseType");
    }

    #[test]
    #[should_panic]
    fn empty_string() {
        ReleaseType::from_str("").unwrap();
    }
}