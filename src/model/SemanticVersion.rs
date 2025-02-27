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
                SemanticVersion { major, minor, patch, release }
        }
}

pub fn compose_new_semantic_version(version: String, release: String) -> SemanticVersion {
        let tokenized_version: Vec<String> = split_string_using_delimiter(format!("{}.0.0", version), '.');
        return SemanticVersion::new(
                        tokenized_version[0], 
                        tokenized_version[1], 
                        tokenized_version[2], 
                        release
                );
}