use std::collections::HashMap;

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

use crate::commons::string::split_string_using_delimiter;

use super::{semantic_version::SemanticVersion, update::Update};
use super::enums::{release_type::ReleaseType, severity::Severity};
use std::str::FromStr;

pub struct DataModelBuilder{
        partitions_severity: HashMap<String, Severity>,
        partitions_type: HashMap<String, ReleaseType>,
        partitions_date: HashMap<String, DateTime<Utc>>,
        updates_by_partition: HashMap<String, Update>,
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

                let partition: &String = &splitted_str[0];
                let severity: Severity = Severity::from_str(&splitted_str[2]).unwrap();
                let datetime: &str = &format!("{} {}", splitted_str[4], splitted_str[5]);
                let release_ts: DateTime<Utc> = NaiveDateTime::parse_from_str(datetime, "%F %X")
                        .unwrap().and_utc();
                
                self.update_partition_details(partition, &severity, &release_ts);
                
                let signature: &String = &splitted_str[3];

                let installed_version = compose_new_semantic_version(
                        &installed_info[0],
                        &installed_info[1]
                );
                
                let release_type: ReleaseType = compare(&update_version, &installed_version);
                
                match release_type {
                        ReleaseType::Major => self._major_count += 1,
                        ReleaseType::Minor => self._minor_count += 1,
                        ReleaseType::Patch => self._patch_count += 1,
                        ReleaseType::Repack => self._release_count += 1
                }

                let result: Update = Update::new(
                        partition.clone(), release_type, severity, 
                        installed_version, update_version, name.to_string(),
                        release_ts
                );

                self._updates.push(result);
        }
}