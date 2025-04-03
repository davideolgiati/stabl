use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum SemanticVersion {
    Repack,
    Patch,
    Minor,
    Major
}

impl<'a> From<&'a str> for SemanticVersion {
    fn from(input: &'a str) -> Self {
        assert!(!input.is_empty());
    
        match input.to_lowercase().as_str() {
            "security"      => SemanticVersion::Patch,
            "bugfix"        => SemanticVersion::Patch,
            "enhancement"   => SemanticVersion::Minor,
            "unspecified"   => SemanticVersion::Major,
            _               => panic!("'{}' is not a valid value for ReleaseType", input),
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
        let output = SemanticVersion::from(input);

        assert!(output == expected);
    }

    #[test]
    fn happy_path_new_minor() {
        let input: &str = "enhancement";
        let expected: SemanticVersion = SemanticVersion::Minor;
        let output = SemanticVersion::from(input);

        assert!(output == expected);
    }

    #[test]
    fn happy_path_new_patch() {
        let input: &str = "bugfix";
        let expected: SemanticVersion = SemanticVersion::Patch;
        let output = SemanticVersion::from(input);

        assert!(output == expected);
    }

    #[test]
    fn happy_path_new_patch_security() {
        let input: &str = "security";
        let expected: SemanticVersion = SemanticVersion::Patch;
        let output = SemanticVersion::from(input);

        assert!(output == expected);
    }

    #[test]
    fn print_major() {
        let input: &str = "unspecified";
        let expected: &str = "MAJOR";
        let output = SemanticVersion::from(input);

        assert!(format!("{}", output) == expected);
    }

    #[test]
    fn print_minor() {
        let input: &str = "enhancement";
        let expected: &str = "MINOR";
        let output = SemanticVersion::from(input);

        assert!(format!("{}", output) == expected);
    }

    #[test]
    fn print_patch() {
        let input: &str = "bugfix";
        let expected: &str = "PATCH";
        let output = SemanticVersion::from(input);

        assert!(format!("{}", output) == expected);
    }

    #[test]
    fn print_patch_security() {
        let input: &str = "security";
        let expected: &str = "PATCH";
        let output = SemanticVersion::from(input);

        assert!(format!("{}", output) == expected);
    }

    #[test]
    fn print_release_repack() {
        let expected = "REPACK";
        let output = SemanticVersion::Repack;

        assert!(format!("{}", output) == expected);
    }

    #[test]
    #[should_panic]
    fn panic_unknown_string() {
        let input: &str = "major";
        let _ = SemanticVersion::from(input);
    }

    #[test]
    #[should_panic]
    fn panic_empty_string() {
        let _ = SemanticVersion::from("");
    }
}