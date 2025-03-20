use std::collections::HashMap;

use chrono::{DateTime, NaiveDateTime, Utc};

use crate::commons::string::split_string_using_delimiter;

use super::{semantic_version::SemanticVersion, update::Update};
use super::enums::{release_type::ReleaseType, severity::Severity};
use std::str::FromStr;

pub struct DataModelBuilder{
        partitions_severity: HashMap<String, Severity>,
        partitions_type: HashMap<String, ReleaseType>,
        partitions_date: HashMap<String, DateTime<Utc>>,
        updates_by_partition: HashMap<String, String>,
        updates_version: HashMap<String, SemanticVersion>
}

impl DataModelBuilder {
        pub fn new() -> DataModelBuilder {
                DataModelBuilder { 
                        partitions_severity: HashMap::new(), 
                        partitions_type: HashMap::new(), 
                        partitions_date: HashMap::new(), 
                        updates_by_partition: HashMap::new(), 
                        updates_version: HashMap::new() 
                }
        }

        fn update_partition_details(&mut self, id: &String, severity: &Severity, release_ts: &DateTime<Utc>) {
                if !self.partitions_date.contains_key(id) {
                        self.partitions_date.insert(id.clone(), *release_ts);
                } else {
                        let current_release_ts: &DateTime<Utc> = 
                                self.partitions_date.get(id).unwrap();

                        if release_ts > current_release_ts {
                                *self.partitions_date.get_mut(id).unwrap() = *release_ts;
                        } 
                }

                if !self.partitions_severity.contains_key(id) {
                        self.partitions_severity.insert(id.clone(), severity.clone());
                } else {
                        let current_severity: &Severity = self.partitions_severity
                                .get(id).unwrap();

                        if severity > current_severity {
                                *self.partitions_severity.get_mut(id).unwrap() = severity.clone();
                        }
                }
        }

        pub fn add_dnf_output(&mut self, line: String) {
                assert!(!line.is_empty());

                let splitted_str = split_string_using_delimiter(line.to_owned(), " ");

                assert!(splitted_str.len() == 6);
                
                let signature: &String = &splitted_str[3];

                let partition: &String = &splitted_str[0];
                let severity: Severity = Severity::from_str(&splitted_str[2]).unwrap();
                let datetime: &str = &format!("{} {}", splitted_str[4], splitted_str[5]);
                let release_ts: DateTime<Utc> = NaiveDateTime::parse_from_str(datetime, "%F %X")
                        .unwrap().and_utc();
                
                self.update_partition_details(partition, &severity, &release_ts);
                self.updates_by_partition.insert(signature.clone(), partition.clone());
        }


}