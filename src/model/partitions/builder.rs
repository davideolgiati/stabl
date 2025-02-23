use std::collections::HashMap;

use crate::model::partitions::partition::Partition;
use crate::model::update::Update;

pub struct PartitionBuilder {
        _partitions: HashMap<String, Partition>,
        _updates_index: HashMap<String, (String, usize)>
}

impl PartitionBuilder {
        pub fn new() -> PartitionBuilder {
                PartitionBuilder {
                        _partitions: HashMap::new(),
                        _updates_index: HashMap::new()
                }
        }

        pub fn add_update(&mut self, update: Update) {                
                let current_partition_id: String = update.get_partition_id().clone();
                let update_signature: String = update.get_signature().clone();
                let updated_partition: Partition = self.update_partition(update);
                let update_position: usize = updated_partition.get_signatures().len() - 1;

                self._partitions.insert(current_partition_id.clone(), updated_partition);
                self._updates_index.insert(update_signature, (current_partition_id, update_position));      
        }

        fn update_partition(&self, update: Update) -> Partition {
                let partition_id: String = update.get_partition_id().clone();
                let mut current_partition: Partition = self.get_partition_by_id(partition_id);

                current_partition.add_update(update.clone());

                return current_partition.clone()
        }

        fn get_partition_by_id(&self, partition_id: String) -> Partition {
                return self._partitions
                        .get(&partition_id)
                        .unwrap_or(&Partition::new())
                        .clone();
        }

        pub fn build(self) -> HashMap<String, Partition> {
                return self._partitions.clone();
        }
}