use std::collections::HashMap;
use crate::commons::string::split_string_using_delimiter;
use crate::model::enums::severity::Severity;
use crate::model::semantic_version::{compare_version, compose_new_semantic_version};
use crate::Update;
use crate::model::enums::release_type::ReleaseType;

use std::str::FromStr;

pub struct UpdateBuilder {
        _repository_info: HashMap<String, Vec<String>>,
        _installed_info: HashMap<String, Vec<String>>,
        _major_count: i16,
        _minor_count: i16,
        _patch_count: i16,
        _release_count: i16
}

impl UpdateBuilder {
        pub fn new(
                repository_info: &HashMap<String, Vec<String>>, 
                installed_info: &HashMap<String, Vec<String>>
        ) -> UpdateBuilder {
                UpdateBuilder{
                        _repository_info: repository_info.clone(),
                        _installed_info: installed_info.clone(),
                        _major_count: 0,
                        _minor_count: 0,
                        _patch_count: 0,
                        _release_count: 0
                }
        }

        pub fn check_dnf_output_valididty(&self, stdout: String) -> bool {
                assert!(!stdout.is_empty());

                let splitted_str = split_string_using_delimiter(stdout, " ");
                assert!(splitted_str.len() == 6);

                let signature: String = splitted_str[3].clone();

                let default_value = Vec::from([String::from("")]);
                let update_info: &Vec<String> = self._repository_info
                        .get(&signature.clone())
                        .unwrap_or(&default_value);

                !update_info[0].is_empty()
        }

        pub fn add_dnf_output(&mut self, stdout: String) -> Update {
                assert!(!stdout.is_empty());

                let splitted_str = split_string_using_delimiter(stdout, " ");

                assert!(splitted_str.len() == 6);

                let partition: String = splitted_str[0].clone();
                let severity: Severity = Severity::from_str(splitted_str[2].as_str()).unwrap();
                let signature: String = splitted_str[3].clone();
                
                let update_info: &Vec<String> = self._repository_info
                        .get(&signature.clone())
                        .unwrap();
        
                let name = update_info[0].clone();
                
                let installed_info: &Vec<String> = self._installed_info
                        .get(&name)
                        .unwrap();


                let update_version = compose_new_semantic_version(
                        update_info[1].clone(),
                        update_info[2].clone()
                );

                let installed_version = compose_new_semantic_version(
                        installed_info[0].clone(),
                        installed_info[1].clone()
                );
                
                let release_type: ReleaseType = compare_version(&update_version, &installed_version);
                
                match release_type {
                        ReleaseType::Major => self._major_count += 1,
                        ReleaseType::Minor => self._minor_count += 1,
                        ReleaseType::Patch => self._patch_count += 1,
                        ReleaseType::Repack => self._release_count += 1
                }

                let result: Update = Update::new(
                        partition, release_type, severity, 
                        installed_version, update_version, name
                );

                result
        }

        pub fn get_major_count(&self) -> &i16 {
                &self._major_count
        }

        pub fn get_minor_count(&self) -> &i16 {
                &self._minor_count
        }

        pub fn get_patch_count(&self) -> &i16 {
                &self._patch_count
        }
        
        pub fn get_release_count(&self) -> &i16 {
                &self._release_count
        }
}