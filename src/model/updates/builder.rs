use std::collections::HashMap;
use crate::Update;

pub struct UpdateBuilder {
        _repository_info: HashMap<String, Vec<String>>,
        _installed_info: HashMap<String, Vec<String>>,
        _updates: Vec<Update>
}

impl UpdateBuilder {
        pub fn new(
                repository_info: HashMap<String, Vec<String>>, 
                installed_info: HashMap<String, Vec<String>>
        ) -> UpdateBuilder {
                return UpdateBuilder{
                        _repository_info: repository_info,
                        _installed_info: installed_info,
                        _updates: Vec::new()
                }
        }

        pub fn from_dnf_output(line: String) -> Update {
                assert!(stdout != "");

                let splitted_str = split_string_using_delimiter(stdout, " ");

                assert!(splitted_str.len() == 6);

                let partition: String = splitted_str[0].clone();
                let release_type: ReleaseType = ReleaseType::from_str(&splitted_str[1]).unwrap();
                let severity: Severity = Severity::from_str(&splitted_str[2]).unwrap();
                let signature: String = splitted_str[3].clone();

                let update_info: &Vec<String> = _repository_info
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

                return result;
        }
}