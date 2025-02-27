use crate::model::enums::release_type::ReleaseType;
use crate::model::enums::severity::Severity;
use crate::commons::string::split_string_using_delimiter;

use std::str::FromStr;


#[derive(Clone)]
pub struct Update {
        _partition: String,
        _release_type: ReleaseType,
        _severity: Severity,
        _signature: String,
        _version: String,
        _release: String,
        _name: String
}


impl Update {
        pub fn new(
                partition: String, 
                release_type: ReleaseType, 
                severity: Severity, 
                signature: String,
                version: String,
                release: String,
                name: String
        ) -> Update { 
                Update {
                        _partition: partition,
                        _release_type: release_type,
                        _severity: severity,
                        _signature: signature,
                        _version: version,
                        _release: release,
                        _name: name
                }
        }

        pub fn get_partition_id(&self) -> &String {
                return &self._partition;
        }

        pub fn get_release_type(&self) -> &ReleaseType {
                return &self._release_type;
        }

        pub fn get_severity(&self) -> &Severity {
                return &self._severity;
        }
        
        pub fn get_signature(&self) -> &String {
                return &self._signature;
        }

        pub fn get_version(&self) -> &String {
                return &self._version;
        }

        pub fn get_release(&self) -> &String {
                return &self._release;
        }

        pub fn get_name(&self) -> &String {
                return &self._name
        }
}