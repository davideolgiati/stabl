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

#[derive(Default)]
pub struct ModelBuilder<'a>{
        partitions_severities: HashMap<&'a str, SecurityClassification>,
        partitions_release_types: HashMap<&'a str, SemanticVersion>,
        partitions_publication_datetimes: HashMap<&'a str, DateTime<Utc>>,
        updates_by_partition: HashMap<&'a str, &'a str>,
        packages_details: HashMap<&'a str, (&'a str, VersionTag)>,
        updates_list: Vec<Update>
}

impl<'a> ModelBuilder<'a> {
        pub fn new() -> ModelBuilder<'a> {
                Self::default()
        }

        fn update_severity(&mut self, partition_id: &'a str, new_severity_level_str: &str) {
                let new_severity_level: SecurityClassification = SecurityClassification::from(new_severity_level_str);

                if let Some(current_severity_level) = self.partitions_severities.get_mut(partition_id) {
                        if new_severity_level > *current_severity_level {
                                *current_severity_level = new_severity_level;
                        }
                } else {
                        self.partitions_severities.insert(partition_id, new_severity_level);
                }
        }

        fn update_publish_datetime(&mut self, partition_id: &'a str, new_publish_date_str: &str, new_publish_time_str: &str) {
                let new_publish_datetime_str: &str = &format!("{} {}", new_publish_date_str, new_publish_time_str);
                let new_publish_datetime: DateTime<Utc> = NaiveDateTime::parse_from_str(new_publish_datetime_str, "%F %X")
                        .unwrap().and_utc();

                if let Some(current_publish_datetime ) = self.partitions_publication_datetimes.get_mut(partition_id) {
                        if new_publish_datetime > *current_publish_datetime {
                                *current_publish_datetime = new_publish_datetime;
                        } 
                } else {
                        self.partitions_publication_datetimes.insert(partition_id, new_publish_datetime);
                }
        }

        fn update_release_type(&mut self, partition_id: &'a str, new_release_type_str: &str) {
                let new_release_type: SemanticVersion = SemanticVersion::from(new_release_type_str);

                if let Some(current_release_type ) = self.partitions_release_types.get_mut(partition_id) {
                        if new_release_type > *current_release_type {
                                *current_release_type = new_release_type;
                        } 
                } else {
                        self.partitions_release_types.insert(partition_id, new_release_type);
                }
        }

        pub fn add_updateinfo_output_line(&mut self, line: &'a str) {
                assert!(!line.is_empty());

                let splitted_str = split_string(line, " ");
                assert!(splitted_str.len() == 6);
                
                let partition_id: &str = splitted_str[0];
                let new_release_type_str: &str = splitted_str[1];
                let new_severity_level_str: &str = splitted_str[2];
                let signature: &str = splitted_str[3];
                let new_publish_date_str: &str = splitted_str[4];
                let new_publish_time_str: &str = splitted_str[5];

                self.update_release_type(partition_id, new_release_type_str);
                self.update_severity(partition_id, new_severity_level_str);
                self.update_publish_datetime(partition_id, new_publish_date_str, new_publish_time_str);
                self.updates_by_partition.insert(signature, partition_id);
        }

        pub fn add_repoquery_output(&mut self, line: &'a str) {
                assert!(!line.is_empty());

                let splitted_str = split_string(line, "|#|");

                assert!(splitted_str.len() == 5);

                let name: &str = splitted_str[0];
                let version_str: &str = splitted_str[1];
                let release_str: &str = splitted_str[2];

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

                let release: SemanticVersion = compare(&current_version, &update_detail.1);
                let current_partition_release: &SemanticVersion = self.partitions_release_types.get(&update_detail.0).unwrap_or_else(|| panic!("{}\n {:?}", update_detail.0, self.partitions_release_types));

                if &release > current_partition_release {
                        *self.partitions_release_types.get_mut(&update_detail.0).unwrap() = release;
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
                
                let partitions: Vec<Partition> = self.partitions_severities
                        .iter()
                        .filter(|(id, _)| updates.contains_key(**id))
                        .map(|(id, severity)| {
                                        let date = self.partitions_publication_datetimes.get(id).unwrap();
                                        let release_type = self.partitions_release_types.get(id).unwrap();

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