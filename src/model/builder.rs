use std::collections::HashMap;

use chrono::{DateTime, NaiveDateTime, Utc};

use crate::commons::string::split_string;
use crate::model::version_tag::compare;

use super::partition::Partition;
use crate::model::update::Update;
use super::{version_tag::VersionTag, semantic_version::SemanticVersion, security_classification::SecurityClassification};
use std::str::FromStr;

#[derive(Default)]
pub struct ModelBuilder<'a>{
        partitions_severity: HashMap<&'a str, SecurityClassification>,
        partitions_type: HashMap<&'a str, SemanticVersion>,
        partitions_date: HashMap<&'a str, DateTime<Utc>>,
        updates_by_partition: HashMap<&'a str, &'a str>,
        packages_details: HashMap<&'a str, (&'a str, VersionTag)>,
        updates_list: Vec<Update>
}

impl<'a> ModelBuilder<'a> {
        pub fn new() -> ModelBuilder<'a> {
                Self::default()
        }

        fn update_partition_details(&mut self, id: &'a str, severity: &SecurityClassification, release_ts: &DateTime<Utc>, release_type: &SemanticVersion) {
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
                        let current_severity: &SecurityClassification = self.partitions_severity
                                .get(id).unwrap();

                        if severity > current_severity {
                                *self.partitions_severity.get_mut(id).unwrap() = severity.clone();
                        }
                }

                if !self.partitions_type.contains_key(id) {
                        self.partitions_type.insert(id, release_type.clone());
                } else {
                        let current_release_type: &SemanticVersion = self.partitions_type
                                .get(id).unwrap();

                        if release_type > current_release_type {
                                *self.partitions_type.get_mut(id).unwrap() = release_type.clone();
                        }
                }
        }

        pub fn add_updateinfo_output_line(&mut self, line: &'a str) {
                assert!(!line.is_empty());

                let splitted_str = split_string(line, " ");
                assert!(splitted_str.len() == 6);
                
                let signature: &str = splitted_str[3];
                let partition: &str = splitted_str[0];
                let release_type: SemanticVersion = SemanticVersion::from(splitted_str[1]);
                let security_classification: SecurityClassification = SecurityClassification::from_str(splitted_str[2]).unwrap();
                let datetime: &str = &format!("{} {}", splitted_str[4], splitted_str[5]);
                let release_ts: DateTime<Utc> = NaiveDateTime::parse_from_str(datetime, "%F %X")
                        .unwrap().and_utc();
                
                self.update_partition_details(partition, &security_classification, &release_ts, &release_type);
                self.updates_by_partition.insert(signature, partition);
        }

        pub fn add_repoquery_output(&mut self, line: &'a str) {
                assert!(!line.is_empty());

                let splitted_str = split_string(line, "|#|");

                assert!(splitted_str.len() == 5);

                let name: &str = splitted_str[0];
                let version_str: &str = splitted_str[1];
                let release_str: &str = splitted_str[2];

                //println!("[*] {} {}-{}", name, version_str, release_str);

                let version: VersionTag = VersionTag::new(
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

                let splitted_str = split_string(line, "|#|");

                assert!(splitted_str.len() == 3);

                let name: &str = splitted_str[0];
                let version_str: &str = splitted_str[1];
                let release_str: &str = splitted_str[2];

                let current_version: VersionTag = VersionTag::new(
                        version_str, release_str
                );

                let update_detail: &(&str, VersionTag) = self.packages_details.get(name).unwrap();

                //println!("[*] {} {} -> {}", name, current_version, update_detail.1);

                let release: SemanticVersion = compare(&current_version, &update_detail.1);
                let current_partition_release: &SemanticVersion = self.partitions_type.get(&update_detail.0).unwrap_or_else(|| panic!("{}\n {:?}", update_detail.0, self.partitions_type));

                if &release > current_partition_release {
                        *self.partitions_type.get_mut(&update_detail.0).unwrap() = release;
                }
        }

        pub fn build(self) -> (Vec<Partition>, HashMap<String, Vec<Update>>) {
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
                
                let partitions: Vec<Partition> = self.partitions_severity
                        .iter()
                        .filter(|(id, _)| updates.contains_key(**id))
                        .map(|(id, severity)| {
                                        let date = self.partitions_date.get(id).unwrap();
                                        let release_type = self.partitions_type.get(id).unwrap();

                                        Partition::new(
                                                id.to_string(), release_type.clone(), 
                                                severity.clone(), *date
                                        )
                        })
                        .collect();


                (partitions, updates)
        }

}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn happy_path_new_datamodel() {
        let mut data_model: ModelBuilder<'_> = ModelBuilder::new();
        let dnf_updates_list = ["FEDORA-2025-1a0c45a564 enhancement None                   vim-minimal-2:9.1.1227-1.fc41.x86_64 2025-03-23 01:13:07",
                "FEDORA-2025-1a0c45a564 enhancement None                           xxd-2:9.1.1227-1.fc41.x86_64 2025-03-23 01:13:07"];
        let repository_update_details = ["vim-minimal|#|9.1.1227|#|1.fc41|#|vim-minimal-2:9.1.1227-1.fc41.x86_64|#|vim-minimal-9.1.1227-1.fc41.x86_64",
                "xxd|#|9.1.1227|#|1.fc41|#|xxd-2:9.1.1227-1.fc41.x86_64|#|xxd-9.1.1227-1.fc41.x86_64"];
        let packages_names = ["vim-minimal|#|9.1.1202|#|1.fc41",
                "xxd|#|9.1.1202|#|1.fc41"];

        let part_date = NaiveDateTime::parse_from_str("2025-03-23 01:13:07", "%F %X").unwrap().and_utc();
        let expected_partitions = vec![Partition::new("FEDORA-2025-1a0c45a564".to_string(), SemanticVersion::Minor, SecurityClassification::None, part_date)];

        dnf_updates_list.iter().for_each(|line| data_model.add_updateinfo_output_line(line));
        repository_update_details.iter().for_each(|line| data_model.add_repoquery_output(line));
        packages_names.iter().for_each(|line| data_model.add_rpm_output(line));

        let (partitions, _updates) = data_model.build();

        assert!(partitions == expected_partitions);
    }
}