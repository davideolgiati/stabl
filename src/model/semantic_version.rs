use crate::commons::string::split_string_using_delimiter;
use std::fmt::{self, Display, Formatter};

use super::release_type::ReleaseType;

#[derive(Clone, Debug, PartialEq)]
pub struct SemanticVersion {
        _major: String,
        _minor: String,
        _patch: String,
        _release: String
}

impl SemanticVersion {
        pub fn new(
                major: &str,
                minor: &str, 
                patch: &str,
                release: &str
        ) -> SemanticVersion {
                assert!(!major.is_empty());
                assert!(!minor.is_empty());
                assert!(!patch.is_empty());
                assert!(!release.is_empty());

                SemanticVersion { 
                        _major: major.to_owned(), 
                        _minor: minor.to_owned(), 
                        _patch: patch.to_owned(), 
                        _release: release.to_owned()
                }
        }
}

pub fn compose_new_semantic_version(version: &str, release: &str) -> SemanticVersion {
        let padded_string: String = format!("{}.0.0", version);
        let version_tokens: Vec<&str> = split_string_using_delimiter(&padded_string, ".");
        SemanticVersion::new(
                version_tokens[0], 
                version_tokens[1], 
                version_tokens[2], 
                release
        )
}

pub fn compare(current: &SemanticVersion, update: &SemanticVersion) -> ReleaseType {
        if current._major != update._major {
                return ReleaseType::Major
        }

        if current._minor != update._minor {
                return ReleaseType::Minor
        }

        if current._patch != update._patch {
                return ReleaseType::Patch
        }

        if current._release != update._release {
                return ReleaseType::Repack
        }

        panic!("current and update cannot be equal!")
}

impl Display for SemanticVersion {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(
                        f, "{}.{}.{}-{}", 
                        self._major, self._minor, 
                        self._patch, self._release
                )
        }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn happy_path_compose_full_version() {
        let version: &str = "1.1.1";
        let release: &str = "1";
        let expected = SemanticVersion::new(
                "1", 
                "1", 
                "1", 
                "1"
        );

        let actual: SemanticVersion = compose_new_semantic_version(version, release);

        assert_eq!(actual, expected);
    }

    #[test]
    fn happy_path_compose_minor_only_version() {
        let version: &str = "1.1";
        let release: &str = "1";
        let expected = SemanticVersion::new(
                "1", 
                "1", 
                "0", 
                "1"
        );

        let actual: SemanticVersion = compose_new_semantic_version(version, release);

        assert_eq!(actual, expected);
    }

    #[test]
    fn happy_path_compose_major_only_version() {
        let version: &str = "1";
        let release: &str = "1";
        let expected = SemanticVersion::new(
                "1", 
                "0", 
                "0", 
                "1"
        );

        let actual: SemanticVersion = compose_new_semantic_version(version, release);

        assert_eq!(actual, expected);
    }

    #[test]
    fn happy_path_compare_major() {
        let update = SemanticVersion::new(
                "2", 
                "0", 
                "0", 
                "1"
        );
        let installed = SemanticVersion::new(
                "1", 
                "0", 
                "0", 
                "1"
        );

        let actual: ReleaseType = compare(&installed, &update);
        let expected: ReleaseType = ReleaseType::Major;

        assert_eq!(actual, expected);
    }

    #[test]
    fn happy_path_compare_minor() {
        let update = SemanticVersion::new(
                "1", 
                "1", 
                "0", 
                "1"
        );
        let installed = SemanticVersion::new(
                "1", 
                "0", 
                "0", 
                "1"
        );

        let actual: ReleaseType = compare(&installed, &update);
        let expected: ReleaseType = ReleaseType::Minor;

        assert_eq!(actual, expected);
    }

    #[test]
    fn happy_path_compare_patch() {
        let update = SemanticVersion::new(
                "1", 
                "0", 
                "1", 
                "1"
        );
        let installed = SemanticVersion::new(
                "1", 
                "0", 
                "0", 
                "1"
        );

        let actual: ReleaseType = compare(&installed, &update);
        let expected: ReleaseType = ReleaseType::Patch;

        assert_eq!(actual, expected);
    }

    #[test]
    fn happy_path_compare_repack() {
        let update = SemanticVersion::new(
                "1", 
                "0", 
                "0", 
                "2"
        );
        let installed = SemanticVersion::new(
                "1", 
                "0", 
                "0", 
                "1"
        );

        let actual: ReleaseType = compare(&installed, &update);
        let expected: ReleaseType = ReleaseType::Repack;

        assert_eq!(actual, expected);
    }

    #[test]
    fn print_version() {
        let version: &str = "1.0.0";
        let release: &str = "1";
        let expected: &str = "1.0.0-1";
        let output =  compose_new_semantic_version(version, release);

        assert_eq!(format!("{}", output), expected);
    }

    #[test]
    #[should_panic]
    fn panic_compare_equal_version() {
        let update = SemanticVersion::new(
                "1", 
                "0", 
                "0", 
                "1"
        );
        let installed = SemanticVersion::new(
                "1", 
                "0", 
                "0", 
                "1"
        );

        compare(&installed, &update);
    }

    #[test]
    #[should_panic]
    fn panic_empty_major() {
        SemanticVersion::new(
                "", 
                "0", 
                "0", 
                "1"
        );
    }

    #[test]
    #[should_panic]
    fn panic_empty_minor() {
        SemanticVersion::new(
                "1", 
                "", 
                "0", 
                "1"
        );
    }

    #[test]
    #[should_panic]
    fn panic_empty_patch() {
        SemanticVersion::new(
                "1", 
                "0", 
                "", 
                "1"
        );
    }

    #[test]
    #[should_panic]
    fn panic_empty_release() {
        SemanticVersion::new(
                "1", 
                "0", 
                "0", 
                ""
        );
    }
}