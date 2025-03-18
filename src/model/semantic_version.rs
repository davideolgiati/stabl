use crate::commons::string::split_string_using_delimiter;
use std::fmt::{self, Display, Formatter};

use super::enums::release_type::ReleaseType;

#[derive(Clone)]
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
        let version_tokens: Vec<String> = split_string_using_delimiter(padded_string, ".");
        SemanticVersion::new(
                &version_tokens[0], 
                &version_tokens[1], 
                &version_tokens[2], 
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

        ReleaseType::Repack
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