use std::str::FromStr;
use std::fmt::{self, Display, Formatter};

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, PartialOrd)]
pub enum SemanticVersion {
    Repack,
    Patch,
    Minor,
    Major
}

pub fn get_super(value: &SemanticVersion) -> SemanticVersion {
    match *value {
        SemanticVersion::Major => SemanticVersion::Major,
        SemanticVersion::Minor => SemanticVersion::Major,
        SemanticVersion::Patch => SemanticVersion::Minor,
        SemanticVersion::Repack => SemanticVersion::Patch
    }
}

impl FromStr for SemanticVersion {
    type Err = String;

    fn from_str(input: &str) -> Result<SemanticVersion, Self::Err> {
        assert!(!input.is_empty());
    
        match input.to_lowercase().as_str() {
            "security"      => Ok(SemanticVersion::Patch),
            "bugfix"        => Ok(SemanticVersion::Patch),
            "enhancement"   => Ok(SemanticVersion::Minor),
            "unspecified"   => Ok(SemanticVersion::Major),
            _               => Err(format!("'{}' is not a valid value for ReleaseType", input)),
        }
    }
}

impl Display for SemanticVersion {
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
    fn happy_path_new_major() {
        let input: &str = "unspecified";
        let expected: SemanticVersion = SemanticVersion::Major;
        let output = SemanticVersion::from_str(input).unwrap();

        assert!(output == expected);
    }

    #[test]
    fn happy_path_new_minor() {
        let input: &str = "enhancement";
        let expected: SemanticVersion = SemanticVersion::Minor;
        let output = SemanticVersion::from_str(input).unwrap();

        assert!(output == expected);
    }

    #[test]
    fn happy_path_new_patch() {
        let input: &str = "bugfix";
        let expected: SemanticVersion = SemanticVersion::Patch;
        let output = SemanticVersion::from_str(input).unwrap();

        assert!(output == expected);
    }

    #[test]
    fn happy_path_new_patch_security() {
        let input: &str = "security";
        let expected: SemanticVersion = SemanticVersion::Patch;
        let output = SemanticVersion::from_str(input).unwrap();

        assert!(output == expected);
    }

    #[test]
    fn happy_path_get_super_for_major() {
        let input:SemanticVersion = SemanticVersion::Major;
        let expected: SemanticVersion  = SemanticVersion::Major;
        let output: SemanticVersion = get_super(&input);

        assert!(output == expected);
    }

    #[test]
    fn happy_path_get_super_for_minor() {
        let input: SemanticVersion  = SemanticVersion::Minor;
        let expected: SemanticVersion  = SemanticVersion::Major;
        let output: SemanticVersion = get_super(&input);

        assert!(output == expected);
    }

    #[test]
    fn happy_path_get_super_for_patch() {
        let input: SemanticVersion = SemanticVersion::Patch;
        let expected: SemanticVersion  = SemanticVersion::Minor;
        let output: SemanticVersion = get_super(&input);

        assert!(output == expected);
    }

    #[test]
    fn happy_path_get_super_for_repack() {
        let input: SemanticVersion = SemanticVersion::Repack;
        let expected: SemanticVersion  = SemanticVersion::Patch;
        let output: SemanticVersion = get_super(&input);

        assert!(output == expected);
    }

    #[test]
    fn print_major() {
        let input: &str = "unspecified";
        let expected: &str = "MAJOR";
        let output = SemanticVersion::from_str(input).unwrap();

        assert!(format!("{}", output) == expected);
    }

    #[test]
    fn print_minor() {
        let input: &str = "enhancement";
        let expected: &str = "MINOR";
        let output = SemanticVersion::from_str(input).unwrap();

        assert!(format!("{}", output) == expected);
    }

    #[test]
    fn print_patch() {
        let input: &str = "bugfix";
        let expected: &str = "PATCH";
        let output = SemanticVersion::from_str(input).unwrap();

        assert!(format!("{}", output) == expected);
    }

    #[test]
    fn print_patch_security() {
        let input: &str = "security";
        let expected: &str = "PATCH";
        let output = SemanticVersion::from_str(input).unwrap();

        assert!(format!("{}", output) == expected);
    }

    #[test]
    fn panic_unknown_string() {
        let input: &str = "major";
        let output = SemanticVersion::from_str(input);

        assert!(output.is_err(), "'major' is not a valid value for ReleaseType");
    }

    #[test]
    #[should_panic]
    fn panic_empty_string() {
        SemanticVersion::from_str("").unwrap();
    }
}