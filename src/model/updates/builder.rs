use std::collections::HashMap;
use crate::Update;

pub struct UpdateBuilder {
        _repository_info: HashMap<String, (String, String, String)>,
        _installed_info: HashMap<String, (String, String, String)>,
        _updates: Vec<Update>
}

impl UpdateBuilder {
        pub fn new(
                repository_info: HashMap<String, (String, String, String)>, 
                installed_info: HashMap<String, (String, String, String)>
        ) -> UpdateBuilder {
                return UpdateBuilder{
                        _repository_info: repository_info,
                        _installed_info: installed_info,
                        _updates: Vec::new()
                }
        }
}