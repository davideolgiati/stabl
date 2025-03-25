use std::collections::HashMap;

use chrono::{DateTime, NaiveDateTime, Utc};

use crate::commons::string::split_string_using_delimiter;
use crate::model::semantic_version::{compare, compose_new_semantic_version};

use super::partition::Partition;
use super::{semantic_version::SemanticVersion, update::Update};
use super::enums::{release_type::ReleaseType, severity::Severity};
use std::str::FromStr;

#[derive(Default)]
pub struct DataModelBuilder<'a>{
        partitions_severity: HashMap<&'a str, Severity>,
        partitions_type: HashMap<&'a str, ReleaseType>,
        partitions_date: HashMap<&'a str, DateTime<Utc>>,
        updates_by_partition: HashMap<&'a str, &'a str>,
        packages_details: HashMap<&'a str, (&'a str, SemanticVersion)>,
        updates_list: Vec<Update>
}

impl<'a> DataModelBuilder<'a> {
        pub fn new() -> DataModelBuilder<'a> {
                Self::default()
        }

        fn update_partition_details(&mut self, id: &'a str, severity: &Severity, release_ts: &DateTime<Utc>, release_type: &ReleaseType) {
                if !self.partitions_date.contains_key(id) {
                        self.partitions_date.insert(id, *release_ts);
                } else {
                        let current_release_ts: &DateTime<Utc> = 
                                self.partitions_date.get(id).unwrap();

                        if release_ts > current_release_ts {
                                *self.partitions_date.get_mut(id).unwrap() = *release_ts;
                        } 
                }

                if !self.partitions_severity.contains_key(id) {
                        self.partitions_severity.insert(id, severity.clone());
                } else {
                        let current_severity: &Severity = self.partitions_severity
                                .get(id).unwrap();

                        if severity > current_severity {
                                *self.partitions_severity.get_mut(id).unwrap() = severity.clone();
                        }
                }

                if !self.partitions_type.contains_key(id) {
                        self.partitions_type.insert(id, release_type.clone());
                } else {
                        let current_release_type: &ReleaseType = self.partitions_type
                                .get(id).unwrap();

                        if release_type > current_release_type {
                                *self.partitions_type.get_mut(id).unwrap() = release_type.clone();
                        }
                }
        }

        pub fn add_updateinfo_output(&mut self, line: &'a str) {
                assert!(!line.is_empty());

                let splitted_str = split_string_using_delimiter(line, " ");

                assert!(splitted_str.len() == 6);
                
                let signature: &str = splitted_str[3];

                let partition: &str = splitted_str[0];
                let release_type: ReleaseType = ReleaseType::from_str(splitted_str[1]).unwrap();
                let severity: Severity = Severity::from_str(splitted_str[2]).unwrap();
                let datetime: &str = &format!("{} {}", splitted_str[4], splitted_str[5]);
                let release_ts: DateTime<Utc> = NaiveDateTime::parse_from_str(datetime, "%F %X")
                        .unwrap().and_utc();
                
                self.update_partition_details(partition, &severity, &release_ts, &release_type);
                self.updates_by_partition.insert(signature, partition);
        }

        pub fn add_repoquery_output(&mut self, line: &'a str) {
                assert!(!line.is_empty());

                let splitted_str = split_string_using_delimiter(line, "|#|");

                assert!(splitted_str.len() == 5);

                let name: &str = splitted_str[0];
                let version_str: &str = splitted_str[1];
                let release_str: &str = splitted_str[2];

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

                let new_obj: Update = Update::new(partition.to_string(), version.clone(), name.to_string());

                self.updates_list.push(new_obj);
                self.packages_details.insert(name, (partition, version));
        }

        pub fn add_rpm_output(&mut self, line: &'a str) {
                assert!(!line.is_empty());

                let splitted_str = split_string_using_delimiter(line, "|#|");

                assert!(splitted_str.len() == 3);

                let name: &str = splitted_str[0];
                let version_str: &str = splitted_str[1];
                let release_str: &str = splitted_str[2];

                let current_version: SemanticVersion = compose_new_semantic_version(
                        version_str, release_str
                );

                let update_detail: &(&str, SemanticVersion) = self.packages_details.get(name).unwrap();

                let release: ReleaseType = compare(&current_version, &update_detail.1);
                let current_partition_release: &ReleaseType = self.partitions_type.get(&update_detail.0).unwrap_or_else(|| panic!("{}\n {:?}", update_detail.0, self.partitions_type));

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
                                        id.to_string(), release_type.clone(), 
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