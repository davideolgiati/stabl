use crate::commons::string::split_string;
use std::fmt::{self, Display, Formatter};

use super::semantic_version::SemanticVersion;

#[derive(Clone, Debug, PartialEq)]
pub struct VersionTag {
        _major: String,
        _minor: String,
        _patch: String,
        _release: String
}

impl VersionTag {
        pub fn new(version: &str, release: &str) -> VersionTag {
                assert!(!version.is_empty());
                assert!(!release.is_empty());

                crate::trace!("Parsing version: \"{}\" - release: \"{}\"", version, release);

                let (version_tokens, residuals)  = {
                        if !version.contains(".") {
                                (vec![&version, "0", "0"], "".to_string())
                        } else {
                                let mut splitted_str: Vec<&str> = split_string(version, ".");
                                while splitted_str.len() < 3 {
                                        splitted_str.push("0");
                                }

                                if splitted_str.len() <= 3 {
                                        (splitted_str, "".to_string())
                                } else {
                                        (splitted_str[0..3].to_vec(), splitted_str[3..].join("."))
                                }

                                
                        }
                };

                let fixed_release = {
                        if residuals.is_empty() {
                                release
                        } else {
                                &format!("{}.{}", residuals, release)
                        }
                };

                VersionTag {
                        _major: version_tokens[0].to_string(), 
                        _minor: version_tokens[1].to_string(), 
                        _patch: version_tokens[2].to_string(), 
                        _release: fixed_release.to_owned()
                }
        }
}

pub(crate) fn compare(current: &VersionTag, update: &VersionTag) -> SemanticVersion {
        if current._major != update._major {
                return SemanticVersion::Major
        }

        if current._minor != update._minor {
                return SemanticVersion::Minor
        }

        if current._patch != update._patch {
                return SemanticVersion::Patch
        }

        if current._release != update._release {
                return SemanticVersion::Repack
        }

        panic!("current and update cannot be equal!")
}

impl Display for VersionTag {
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
    fn happy_path_compare() {
        let base = VersionTag::new("1.0.0", "0.fc41");
        let repack = VersionTag::new("1.0.0", "1.fc41");
        let patch = VersionTag::new("1.0.1", "0.fc41");
        let minor = VersionTag::new("1.1.0", "0.fc41");
        let major = VersionTag::new("2.0.0", "0.fc41");

        assert!(compare(&base, &repack) == SemanticVersion::Repack);
        assert!(compare(&base, &patch) == SemanticVersion::Patch);
        assert!(compare(&base, &minor) == SemanticVersion::Minor);
        assert!(compare(&base, &major) == SemanticVersion::Major);
    }

    #[test]
    #[should_panic]
    fn panic_compare_same_version() {
        let base = VersionTag::new("1.0.0", "0.fc41");

        compare(&base, &base);
    }

    #[test]
    fn happy_path_new_version_tag_full() {
        let expected_major = "1";
        let expected_minor = "2";
        let expected_patch = "3";
        let expected_release = "0.fc41";

        let input_version = "1.2.3";
        let input_release = "0.fc41";

        let output = VersionTag::new(input_version, input_release);

        assert!(output._major == expected_major);
        assert!(output._minor == expected_minor);
        assert!(output._patch == expected_patch);
        assert!(output._release == expected_release);
    }

    #[test]
    fn happy_path_new_version_tag_missing_patch() {
        let expected_major = "1";
        let expected_minor = "2";
        let expected_patch = "0";
        let expected_release = "0.fc41";

        let input_version = "1.2";
        let input_release = "0.fc41";

        let output = VersionTag::new(input_version, input_release);

        assert!(output._major == expected_major);
        assert!(output._minor == expected_minor);
        assert!(output._patch == expected_patch);
        assert!(output._release == expected_release);
    }

    #[test]
    fn happy_path_new_version_tag_only_major() {
        let expected_major = "1";
        let expected_minor = "0";
        let expected_patch = "0";
        let expected_release = "0.fc41";

        let input_version = "1";
        let input_release = "0.fc41";

        let output = VersionTag::new(input_version, input_release);

        assert!(output._major == expected_major);
        assert!(output._minor == expected_minor);
        assert!(output._patch == expected_patch);
        assert!(output._release == expected_release);
    }

    #[test]
    fn happy_path_new_version_tag_full_and_release() {
        let expected_major = "1";
        let expected_minor = "2";
        let expected_patch = "3";
        let expected_release = "4.0.fc41";

        let input_version = "1.2.3.4";
        let input_release = "0.fc41";

        let output = VersionTag::new(input_version, input_release);

        assert!(output._major == expected_major);
        assert!(output._minor == expected_minor);
        assert!(output._patch == expected_patch);
        assert!(output._release == expected_release);
    }
}