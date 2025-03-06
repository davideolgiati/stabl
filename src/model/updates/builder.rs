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
        _updates: Vec<Update>,
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
                        _updates: Vec::new(),
                        _major_count: 0,
                        _minor_count: 0,
                        _patch_count: 0,
                        _release_count: 0
                }
        }

        pub fn check_dnf_output_valididty(&self, stdout: &String) -> bool {
                assert!(!stdout.is_empty());

                let splitted_str = split_string_using_delimiter(stdout.to_owned(), " ");
                assert!(splitted_str.len() == 6);

                let signature: String = splitted_str[3].clone();

                let default_value = Vec::from([String::from("")]);
                let update_info: &Vec<String> = self._repository_info
                        .get(&signature.clone())
                        .unwrap_or(&default_value);

                !update_info[0].is_empty()
        }

        pub fn add_dnf_output(&mut self, stdout: &String) {
                assert!(!stdout.is_empty());

                let splitted_str = split_string_using_delimiter(stdout.to_owned(), " ");

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

                self._updates.push(result);
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

        pub fn get_updates(&self) -> &Vec<Update> {
                &self._updates
        }
}

#[cfg(test)]
mod tests {
        use crate::model::semantic_version::SemanticVersion;

        use super::*;
        
        #[test]
        fn happy_path() {
                let update_map = HashMap::from([(
                        "firefox-2.0.0-1.fc41".to_string(), 
                        vec![
                                "firefox".to_string(), 
                                "2.0.0".to_string(), 
                                "1.fc41".to_string()
                        ]
                )]);
                let installed_map = HashMap::from([(
                        "firefox".to_string(), 
                        vec![
                                "1.0.0".to_string(), 
                                "1.fc41".to_string()
                        ]
                )]);
                let mut update_builder = UpdateBuilder::new(
                        &update_map, &installed_map
                );
                let stdout: &str = "Fedora-2025-1234 bugfix None firefox-2.0.0-1.fc41 2025-03-02 02:18:47";

                update_builder.add_dnf_output(&stdout.to_string());

                let output: Update = update_builder.get_updates()[0].clone();

                assert!(*output.get_partition_id() == "Fedora-2025-1234");
                assert!(*output.get_release_type() == ReleaseType::Major);
                assert!(*output.get_severity() == Severity::None);
                assert!(*output.get_name() == "firefox");

                let update_version: &SemanticVersion = output.get_updated_version();
                let installed_version: &SemanticVersion = output.get_installed_version();

                assert!(format!("{}", update_version) == "2.0.0-1.fc41");
                assert!(format!("{}", installed_version) == "1.0.0-1.fc41");
        }
}