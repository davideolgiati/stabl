use crate::model::enums::release_type::ReleaseType;
use crate::model::enums::severity::Severity;
use crate::model::semantic_version::SemanticVersion;

#[derive(Clone)]
pub struct Update {
        _partition: String,
        _release_type: ReleaseType,
        _severity: Severity,
        _installed_version: SemanticVersion,
        _update_version: SemanticVersion,
        _name: String
}


impl Update {
        pub fn new(
                partition: String, 
                release_type: ReleaseType, 
                severity: Severity,
                installed_version: SemanticVersion,
                update_version: SemanticVersion,
                name: String
        ) -> Update { 
                Update {
                        _partition: partition,
                        _release_type: release_type,
                        _severity: severity,
                        _installed_version: installed_version,
                        _update_version: update_version,
                        _name: name
                }
        }

        pub fn get_partition_id(&self) -> &String {
                &self._partition
        }

        pub fn get_release_type(&self) -> &ReleaseType {
                &self._release_type
        }

        pub fn get_severity(&self) -> &Severity {
                &self._severity
        }

        pub fn get_installed_version(&self) -> &SemanticVersion {
                &self._installed_version
        }

        pub fn get_updated_version(&self) -> &SemanticVersion {
                &self._update_version
        }

        pub fn get_name(&self) -> &String {
                &self._name
        }
}