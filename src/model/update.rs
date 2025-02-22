use crate::model::release_type::ReleaseType;
use crate::commons::string::split_string_using_delimiter;

use std::str::FromStr;

#[derive(Clone)]
pub struct Update {
        _partition: String,
        _release_type: ReleaseType,
        _severity: String,
        _signature: String
}


impl Update {
        pub fn new(
                partition: String, 
                release_type: ReleaseType, 
                severity: String, 
                signature: String
        ) -> Update { 
                Update {
                        _partition: partition,
                        _release_type: release_type,
                        _severity: severity,
                        _signature: signature
                }
        }

        pub fn from_dnf_output(stdout: String) -> Update{
                assert!(stdout != "");

                let splitted_str = split_string_using_delimiter(stdout, " ");

                assert!(splitted_str.len() == 6);

                let partition: String = splitted_str[0].clone();
                let release_type: ReleaseType = ReleaseType::from_str(&splitted_str[1]).unwrap();
                let severity: String = splitted_str[2].clone();
                let signature: String = splitted_str[3].clone();

                let result: Update = Update::new(partition, release_type, severity, signature);

                return result;
        }

        pub fn get_partition_id(&self) -> &String {
                &self._partition
        }

        pub fn get_release_type(&self) -> &ReleaseType {
                &self._release_type
        }

        pub fn get_severity(&self) -> &String {
                &self._severity
        }
        
        pub fn get_signature(&self) -> &String {
                &self._signature
        }
}