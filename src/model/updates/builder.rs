use std::collections::HashMap;
use crate::commons::string::split_string_using_delimiter;
use crate::model::enums::severity::Severity;
use crate::Update;
use crate::model::enums::release_type::ReleaseType;

use std::str::FromStr;

pub struct UpdateBuilder {
        _repository_info: HashMap<String, Vec<String>>,
        _installed_info: HashMap<String, Vec<String>>,
        _updates: Vec<Update>
}

impl UpdateBuilder {
        pub fn new(
                repository_info: &HashMap<String, Vec<String>>, 
                installed_info: &HashMap<String, Vec<String>>
        ) -> UpdateBuilder {
                UpdateBuilder{
                        _repository_info: repository_info.clone(),
                        _installed_info: installed_info.clone(),
                        _updates: Vec::new()
                }
        }

        pub fn from_dnf_output(&self, stdout: String) -> Update {
                assert!(!stdout.is_empty());

                let splitted_str = split_string_using_delimiter(stdout, " ");

                assert!(splitted_str.len() == 6);

                let partition: String = splitted_str[0].clone();
                let release_type: ReleaseType = ReleaseType::from_str(splitted_str[1].as_str()).unwrap();
                let severity: Severity = Severity::from_str(splitted_str[2].as_str()).unwrap();
                let signature: String = splitted_str[3].clone();

                let update_info: &Vec<String> = self._repository_info
                        .get(&signature.clone())
                        .unwrap();

                let version = update_info[1].clone();
                let release = update_info[2].clone();
                let name = update_info[0].clone();

                let result: Update = Update::new(
                        partition, release_type, 
                        severity, signature, 
                        version, release, name
                );

                result
        }
}