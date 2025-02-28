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
                major: String,
                minor: String, 
                patch: String,
                release: String
        ) -> SemanticVersion {
                SemanticVersion { 
                        _major: major, 
                        _minor: minor, 
                        _patch: patch, 
                        _release: release 
                }
        }
}

pub fn compose_new_semantic_version(version: String, release: String) -> SemanticVersion {
        let tokenized_version: Vec<String> = split_string_using_delimiter(format!("{}.0.0", version), ".");
        SemanticVersion::new(
                        tokenized_version[0].clone(), 
                        tokenized_version[1].clone(), 
                        tokenized_version[2].clone(), 
                        release
                )
}

pub fn compare_version(high: &SemanticVersion, low: &SemanticVersion) -> ReleaseType {
        if high._major != low._major {
                return ReleaseType::Major
        }

        if high._minor != low._minor {
                return ReleaseType::Minor
        }

        if high._patch != low._patch {
                return ReleaseType::Patch
        }

        ReleaseType::Repack
}

impl Display for SemanticVersion {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "{}.{}.{}-{}", self._major, self._minor, self._patch, self._release)
        }
    }