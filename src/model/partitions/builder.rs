use std::collections::HashMap;

use crate::model::partitions::partition::Partition;
use crate::model::update::Update;

use crate::dnf;

pub struct PartitionBuilder {
        _partitions: HashMap<String, Partition>
}

impl PartitionBuilder {
        pub fn new() -> PartitionBuilder {
                PartitionBuilder {
                        _partitions: HashMap::new()
                }
        }

        pub fn register_update(&mut self, update: Update) {
                let current_partition_id: String = update.get_partition_id().clone();
                let update_signature: String = update.get_signature().clone();
                let updated_partition: Partition = self.update_partition(update);

                self._partitions.insert(current_partition_id.clone(), updated_partition);    
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