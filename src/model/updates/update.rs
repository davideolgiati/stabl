use chrono::{DateTime, Utc};

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
        _name: String,
        _release_ts: DateTime<Utc>
}


impl Update {
        pub fn new(
                partition: String, 
                release_type: ReleaseType, 
                severity: Severity,
                installed_version: SemanticVersion,
                update_version: SemanticVersion,
                name: String,
                release_ts: DateTime<Utc>
        ) -> Update { 
                assert!(!partition.is_empty());
                assert!(!name.is_empty());

                Update {
                        _partition: partition,
                        _release_type: release_type,
                        _severity: severity,
                        _installed_version: installed_version,
                        _update_version: update_version,
                        _name: name,
                        _release_ts: release_ts
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

        pub fn get_release_timestamp(&self) -> &DateTime<Utc> {
                &self._release_ts
        }
}

#[cfg(test)]
mod tests {
        use super::*;
        use crate::model::semantic_version::compose_new_semantic_version;
        
        #[test]
        fn happy_path() {
                let name: String = "firefox".to_string();
                let partition: String = "FEDORA-2025-1234".to_string();
                let severity: Severity = Severity::Critical;
                let release_type: ReleaseType = ReleaseType::Major;
                let installed_version: SemanticVersion = compose_new_semantic_version(
                        "1.0.0", "1.fc41"
                );
                let update_version: SemanticVersion = compose_new_semantic_version(
                        "2.0.0", "2.fc41"
                );

                let output = Update::new(
                        partition.clone(), release_type,
                        severity, installed_version,
                        update_version.clone(), name.clone(),
                        Utc::now()
                );
    
                assert_eq!(*output.get_name(), name);
                assert_eq!(*output.get_partition_id(), partition);
                assert!(*output.get_release_type() == ReleaseType::Major);
                assert!(*output.get_severity() == Severity::Critical);
                assert_eq!(format!("{}", *output.get_installed_version()), "1.0.0-1.fc41");
                assert_eq!(format!("{}", *output.get_updated_version()), "2.0.0-2.fc41");
        }
    
        #[test]
        #[should_panic]
        fn empty_name() {
                let name: String = "".to_string();
                let partition: String = "FEDORA-2025-1234".to_string();
                let severity: Severity = Severity::Critical;
                let release_type: ReleaseType = ReleaseType::Major;
                let installed_version: SemanticVersion = compose_new_semantic_version(
                        "1.0.0", "1.fc41"
                );
                let update_version: SemanticVersion = compose_new_semantic_version(
                        "2.0.0", "2.fc41"
                );

                let _output = Update::new(
                        partition.clone(), release_type,
                        severity, installed_version,
                        update_version.clone(), name.clone(),
                        Utc::now()
                );
        }

        #[test]
        #[should_panic]
        fn empty_partition() {
                let name: String = "firefox".to_string();
                let partition: String = "".to_string();
                let severity: Severity = Severity::Critical;
                let release_type: ReleaseType = ReleaseType::Major;
                let installed_version: SemanticVersion = compose_new_semantic_version(
                        "1.0.0", "1.fc41"
                );
                let update_version: SemanticVersion = compose_new_semantic_version(
                        "2.0.0", "2.fc41"
                );

                let _output = Update::new(
                        partition.clone(), release_type,
                        severity, installed_version,
                        update_version.clone(), name.clone(),
                        Utc::now()
                );
        }

    }