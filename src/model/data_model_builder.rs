use std::collections::HashMap;

use chrono::{DateTime, NaiveDateTime, Utc};

use crate::commons::string::split_string_using_delimiter;
use crate::model::semantic_version::{compare, compose_new_semantic_version};

use super::partition::Partition;
use super::{semantic_version::SemanticVersion, update::Update};
use super::enums::{release_type::ReleaseType, severity::Severity};
use std::str::FromStr;

pub struct DataModelBuilder{
        partitions_severity: HashMap<String, Severity>,
        partitions_type: HashMap<String, ReleaseType>,
        partitions_date: HashMap<String, DateTime<Utc>>,
        updates_by_partition: HashMap<String, String>,
        packages_details: HashMap<String, (String, SemanticVersion)>,
        updates_list: Vec<Update>
}

impl DataModelBuilder {
        pub fn new() -> DataModelBuilder {
                DataModelBuilder { 
                        partitions_severity: HashMap::new(), 
                        partitions_type: HashMap::new(), 
                        partitions_date: HashMap::new(), 
                        updates_by_partition: HashMap::new(), 
                        packages_details: HashMap::new(),
                        updates_list: Vec::new()
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

        pub fn add_updateinfo_output(&mut self, line: String) {
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

        pub fn add_repoquery_output(&mut self, line: String) {
                assert!(!line.is_empty());

                let splitted_str = split_string_using_delimiter(line.to_owned(), "|#|");

                assert!(splitted_str.len() == 5);

                let name: &String = &splitted_str[0];
                let version_str: &String = &splitted_str[1];
                let release_str: &String = &splitted_str[2];

                let version: SemanticVersion = compose_new_semantic_version(
                        version_str, release_str
                );

                let partition = {
                        if self.updates_by_partition.contains_key(&splitted_str[3]) {
                                self.updates_by_partition.get(&splitted_str[3])
                        } else {
                                self.updates_by_partition.get(&splitted_str[4])
                        }
                }.unwrap();

                let new_obj: Update = Update::new(partition.clone(), version.clone(), name.clone());

                self.updates_list.push(new_obj);
                self.packages_details.insert(name.to_string(), (partition.to_owned(), version));
        }

        pub fn add_rpm_output(&mut self, line: String) {
                assert!(!line.is_empty());

                let splitted_str = split_string_using_delimiter(line.to_owned(), "|#|");

                assert!(splitted_str.len() == 3);

                let name: &String = &splitted_str[0];
                let version_str: &String = &splitted_str[1];
                let release_str: &String = &splitted_str[2];

                let current_version: SemanticVersion = compose_new_semantic_version(
                        version_str, release_str
                );

                let update_detail: &(String, SemanticVersion) = self.packages_details.get(name).unwrap();

                let release: ReleaseType = compare(&current_version, &update_detail.1);
                let current_partition_release: &ReleaseType = self.partitions_type.get(&update_detail.0).unwrap();

                if &release > current_partition_release {
                        *self.partitions_type.get_mut(&update_detail.0).unwrap() = release;
                }
        }

        pub fn build(self) -> (Vec<Partition>, HashMap<String, Vec<Update>>) {
                let partitions: Vec<Partition> = self.partitions_severity
                        .iter()
                        .map(|(id, severity)| {
                                let date = self.partitions_date.get(id).unwrap();
                                let release_type = self.partitions_type.get(id).unwrap();

                                Partition::new(
                                        id.clone(), release_type.clone(), 
                                        severity.clone(), *date
                                )
                        })
                        .collect();

                let updates: HashMap<String, Vec<Update>> = 
                        self.updates_list.iter()
                        .fold(HashMap::new(), |mut result, elem| {
                                let partition = elem.get_partition_id();
                                if !result.contains_key(partition) {
                                        result.insert(partition.clone(), vec![elem.clone()]);
                                } else {
                                        result.get_mut(partition).unwrap().push(elem.clone());
                                }

                                result
                        });

                (partitions, updates)
        }

}