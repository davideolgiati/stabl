use crate::model::release_type::ReleseType;

#[derive(Debug)]
pub struct Update {
        name: String,
        release_type: ReleseType,
        severity: String,
        signature: String,
        build_time: String
}


impl Update {
        pub fn new(
                name: String, 
                release_type: ReleseType, 
                severity: String, 
                signature: String, 
                build_time: String
        ) -> Update { 
                Update {
                        name,
                        release_type,
                        severity,
                        signature,
                        build_time
                }
        }
}