from dao.ds.Tie import Tie
from dto.DNFUpdateEntry import DNFUpdateEntry


class UpdatesPartitions():
        def __init__(self):
               self.index = Tie()
               self.partitions = {}

        def add_packages(self, packages_list):
                assert isinstance(packages_list, list)
                for package in packages_list:
                        self.add_package(package)

        def add_package(self, package):
                assert isinstance(package, DNFUpdateEntry)
                assert isinstance(package.key, str)
                assert package.key != ""

                current_partition_id = package.key

                if(not self.index.lookup_key(current_partition_id)):
                        self.index.add_key(current_partition_id)
                        self.partitions[current_partition_id] = []
                
                self.partitions[current_partition_id].append(package)

        def get_partitions(self):
                return self.partitions
                