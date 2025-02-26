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
        _release: String
}


impl Update {
        pub fn new(
                partition: String, 
                release_type: ReleaseType, 
                severity: Severity, 
                signature: String,
                version: String,
                release: String
        ) -> Update { 
                Update {
                        _partition: partition,
                        _release_type: release_type,
                        _severity: severity,
                        _signature: signature,
                        _version: version,
                        _release: release
                }
        }

        pub fn from_dnf_output(stdout: String) -> Update{
                assert!(stdout != "");

                let splitted_str = split_string_using_delimiter(stdout, " ");

                assert!(splitted_str.len() == 6);

                let partition: String = splitted_str[0].clone();
                let release_type: ReleaseType = ReleaseType::from_str(&splitted_str[1]).unwrap();
                let severity: Severity = Severity::from_str(&splitted_str[2]).unwrap();
                let signature: String = splitted_str[3].clone();

                let result: Update = Update::new(
                        partition, release_type, 
                        severity, signature, 
                        String::from(""), 
                        String::from("")
                );

                return result;
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

        pub fn set_version(mut self, version: &String) -> Update {
                self._version = version.clone();
                return self;
        }

        pub fn set_release(mut self, release: &String) -> Update {
                self._release = release.clone();
                return self;
        }
}