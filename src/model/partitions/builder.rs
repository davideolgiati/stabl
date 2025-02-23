use std::collections::HashMap;

use crate::model::update::Update;

pub struct PartitionBuilder {
        _updates_by_partition: HashMap<String, Vec<Update>>,
}

impl PartitionBuilder {
        pub fn new() -> PartitionBuilder {
                PartitionBuilder {
                        _updates_by_partition: HashMap::new()
                }
        }

        pub fn add_update(&mut self, update: Update) {                
                let current_partition_id = update.get_partition_id().clone();
                let updated_partition: Vec<Update> = self.update_partition(update);

                self._updates_by_partition.insert(current_partition_id, updated_partition);         
        }

        fn update_partition(&self, update: Update) -> Vec<Update> {
                let partition_id: String = update.get_partition_id().clone();
                let mut current_partition: Vec<Update> = self.get_partition_by_id(partition_id);

                current_partition.push(update.clone());

                return current_partition.clone()
        }

        fn get_partition_by_id(&self, partition_id: String) -> Vec<Update> {
                return self._updates_by_partition
                        .get(&partition_id)
                        .unwrap_or(&Vec::<Update>::new())
                        .to_vec();
        }

        pub fn build(self) -> HashMap<String, Vec<Update>> {
                return self._updates_by_partition.clone();
        }
}