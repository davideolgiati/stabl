use crate::model::semantic_version::SemanticVersion;

#[derive(Clone)]
pub struct Update {
        _partition: String,
        _version: SemanticVersion,
        _name: String,
}

impl Update {
        pub fn new(partition: String, version: SemanticVersion, name: String) -> Update { 
                assert!(!partition.is_empty());
                assert!(!name.is_empty());

                Update {
                        _partition: partition,
                        _version: version,
                        _name: name
                }
        }

        pub fn get_partition_id(&self) -> &String {
                &self._partition
        }

        pub fn get_version(&self) -> &SemanticVersion {
                &self._version
        }

        pub fn get_name(&self) -> &String {
                &self._name
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
                let version: SemanticVersion = compose_new_semantic_version(
                        "1.0.0", "1.fc41"
                );

                let output = Update::new(
                        partition.clone(), version,
                        name.clone()
                );
    
                assert_eq!(*output.get_name(), name);
                assert_eq!(*output.get_partition_id(), partition);
                assert_eq!(format!("{}", *output.get_version()), "1.0.0-1.fc41");
        }
    
        #[test]
        #[should_panic]
        fn empty_name() {
                let name: String = "".to_string();
                let partition: String = "FEDORA-2025-1234".to_string();
                let version: SemanticVersion = compose_new_semantic_version(
                        "1.0.0", "1.fc41"
                );

                let _output = Update::new(
                        partition.clone(), 
                        version,
                        name.clone()
                );
        }

        #[test]
        #[should_panic]
        fn empty_partition() {
                let name: String = "firefox".to_string();
                let partition: String = "".to_string();
                let version: SemanticVersion = compose_new_semantic_version(
                        "1.0.0", "1.fc41"
                );

                let _output = Update::new(
                        partition.clone(), 
                        version,
                        name.clone()
                );
        }

}