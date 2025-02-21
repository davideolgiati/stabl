use crate::model::release_type::ReleseType;

pub struct Update {
        name: String,
        release_type: ReleseType,
        severity: String,
        signature: String
}


impl Update {
        pub fn new(
                name: String, 
                release_type: ReleseType, 
                severity: String, 
                signature: String
        ) -> Update { 
                Update {
                        name,
                        release_type,
                        severity,
                        signature
                }
        }

        pub fn from_dnf_output(stdout: String) {
                let splitted_str = stdout.split(" ");
                let splitted_str_iter: Vec<String> = splitted_str
                    .into_iter()
                    .clone()
                    .filter(|&str| *str != *"")
                    .map(str::to_string)
                    .collect();
            
                println!("partition: \"{}\"", splitted_str_iter[0]);
                println!("update type: \"{}\"", splitted_str_iter[1]);
                println!("security grade: \"{}\"", splitted_str_iter[2]);
                println!("signature: \"{}\"\n", splitted_str_iter[3]);
        }
}