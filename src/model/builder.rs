use std::collections::HashMap;

use chrono::{DateTime, NaiveDateTime, Utc};

use crate::commons::string::split_string;
use crate::model::version_tag::compare;

use super::partition::Partition;
use crate::model::update::Update;
use super::{
        version_tag::VersionTag, 
        semantic_version::SemanticVersion, 
        security_classification::SecurityClassification
};

struct PartitionDetails {
        severity: SecurityClassification,
        release_type: SemanticVersion,
        publication_datetime: DateTime<Utc>
}

struct UpdateinfoOutput<'a> {
        release_type: &'a str,
        severity_level: &'a str,
        publish_date: &'a str,
        publish_time: &'a str,
}

#[derive(Default)]
pub struct ModelBuilder<'a>{
        partitions: HashMap<&'a str, PartitionDetails>,
        updates_by_partition: HashMap<&'a str, &'a str>,
        packages_details: HashMap<&'a str, (&'a str, VersionTag)>,
        updates_list: Vec<Update>
}

fn compose_datetime_from_string(date: &str, time: &str) -> DateTime<Utc> {
        let datetime: &str = &format!("{} {}", date, time);
        NaiveDateTime::parse_from_str(datetime, "%F %X").unwrap().and_utc()
}

impl<'a> ModelBuilder<'a> {
        pub fn new() -> ModelBuilder<'a> {
                Self::default()
        }

        fn update_partition_details(&mut self, partition_id: &'a str, updateinfo: UpdateinfoOutput<'a>) {
                let input_severity_level = SecurityClassification::from(updateinfo.severity_level);
                let input_release_type = SemanticVersion::from(updateinfo.release_type);
                let input_publish_datetime = compose_datetime_from_string(
                        updateinfo.publish_date, 
                        updateinfo.publish_time
                );

                if let Some(current_partition_details) = self.partitions.get_mut(partition_id) {
                        if input_severity_level > current_partition_details.severity {
                                current_partition_details.severity = input_severity_level;
                        }

                        if input_release_type > current_partition_details.release_type {
                                current_partition_details.release_type = input_release_type;
                        }

                        if input_publish_datetime > current_partition_details.publication_datetime {
                                current_partition_details.publication_datetime = input_publish_datetime;
                        }
                } else {
                        let partition_details = PartitionDetails {
                                severity: input_severity_level,
                                release_type: input_release_type,
                                publication_datetime: input_publish_datetime
                        };

                        self.partitions.insert(partition_id, partition_details);
                }
        }

        pub fn add_updateinfo_output_line(&mut self, updateinfo_stdout: &'a str) {
                assert!(!updateinfo_stdout.is_empty());

                let updateinfo_tokens: Vec<&str> = split_string(updateinfo_stdout, " ");
                assert!(updateinfo_tokens.len() == 6);
                
                let updateinfo_output = UpdateinfoOutput {
                        release_type: updateinfo_tokens[1],
                        severity_level: updateinfo_tokens[2],
                        publish_date: updateinfo_tokens[4],
                        publish_time: updateinfo_tokens[5]
                };

                let partition_id = updateinfo_tokens[0];
                let package_signature = updateinfo_tokens[3];
                
                self.update_partition_details(partition_id, updateinfo_output);

                self.updates_by_partition.insert(
                        package_signature, 
                        partition_id
                );
        }

        pub fn add_repoquery_output(&mut self, repoquery_stdout: &'a str) {
                assert!(!repoquery_stdout.is_empty());

                let repoquery_tokens: Vec<&str> = split_string(repoquery_stdout, "|#|");

                assert!(repoquery_tokens.len() == 5);

                let name: &str = repoquery_tokens[0];
                let version_str: &str = repoquery_tokens[1];
                let release_str: &str = repoquery_tokens[2];

                let version: VersionTag = VersionTag::new(
                        version_str, release_str
                );

                let partition = {
                        if self.updates_by_partition.contains_key(&repoquery_tokens[3]) {
                                self.updates_by_partition.get(&repoquery_tokens[3])
                        } else {
                                self.updates_by_partition.get(&repoquery_tokens[4])
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

                let new_release_type: SemanticVersion = compare(&current_version, &update_detail.1);
                let partition_id = update_detail.0;

                if let Some(current_partition_details) = self.partitions.get_mut(partition_id) {
                        if new_release_type > current_partition_details.release_type {
                                current_partition_details.release_type = new_release_type;
                        }
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
                
                let partitions: Vec<Partition> = self.partitions
                        .iter()
                        .filter(|(id, _)| updates.contains_key(**id))
                        .map(|(id, details)| {
                                        Partition::new(
                                                id.to_string(), 
                                                details.release_type.clone(), 
                                                details.severity.clone(), 
                                                details.publication_datetime
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
    
    #[test]
    fn happy_path_new_datamodel_missing_package_repo() {
        let mut data_model: ModelBuilder<'_> = ModelBuilder::new();
        let dnf_updates_list = ["FEDORA-2025-1a0c45a564 enhancement None                   vim-minimal-2:9.1.1227-1.fc41.x86_64 2025-03-23 01:13:07",
                "FEDORA-2025-1a0c45a563 enhancement None                           xxd-2:9.1.1227-1.fc41.x86_64 2025-03-23 01:13:07"];
        let repository_update_details = ["vim-minimal|#|9.1.1227|#|1.fc41|#|vim-minimal-2:9.1.1227-1.fc41.x86_64|#|vim-minimal-9.1.1227-1.fc41.x86_64"];
        let packages_names = ["vim-minimal|#|9.1.1202|#|1.fc41"];

        let part_date = NaiveDateTime::parse_from_str("2025-03-23 01:13:07", "%F %X").unwrap().and_utc();
        let expected_partitions = vec![Partition::new("FEDORA-2025-1a0c45a564".to_string(), SemanticVersion::Minor, SecurityClassification::None, part_date)];

        dnf_updates_list.iter().for_each(|line| data_model.add_updateinfo_output_line(line));
        repository_update_details.iter().for_each(|line| data_model.add_repoquery_output(line));
        packages_names.iter().for_each(|line| data_model.add_rpm_output(line));

        let (partitions, _updates) = data_model.build();

        assert!(partitions == expected_partitions);
    }
}