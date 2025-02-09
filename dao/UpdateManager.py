from common.logger import log_timed_execution
from dao.DNF import DNF
from dto.enums.UpdateUrgency import UpdateUrgency
from dto.enums.UpdateClassification import UpdateClassification


class UpdateManager():
        max_allowed_version_jump = UpdateClassification.PATCH
        max_skippable_uregency = UpdateUrgency.NONE
        packages = {
                "major": 0,
                "minor": 0,
                "patch": 0,
                "release": 0
        }

        def __init__(self, package_manager):
                assert isinstance(package_manager, DNF)
                self.package_manager = package_manager


        def get_majors_count(self):
                return self.packages['major']
        

        def get_minors_count(self):
                return self.packages['minor']
        
        
        def get_patches_count(self):
                return self.packages['patch']
        
        
        def get_releases_count(self):
                return self.packages['release']
        

        def get_updates_list(self):
                self.updates_partitions = self.package_manager.get_updates_by_partition_id()
                assert isinstance(self.updates_partitions, dict)

                return self.updates_partitions


        def get_suggested_update_partitions(self):
                assert isinstance(self.updates_partitions, dict)
                assert self.updates_partitions != {}

                suggested_updates = []

                for parttion_id, properties in self.updates_partitions.items():                        
                        if(self.evaluate_update_partition(properties)):
                                suggested_updates.append(parttion_id)
                
                return suggested_updates
        

        def evaluate_update_partition(self, partition_property):
                urgency = partition_property["urgency"]
                update_type = partition_property["type"]
                package_count = len(partition_property["packages"])

                allowed_partition = False

                if(urgency > self.max_skippable_uregency):
                        allowed_partition = True

                if(update_type <= self.max_allowed_version_jump):
                        allowed_partition = True

                if (update_type == UpdateClassification.MAJOR):
                        self.packages['major'] += package_count
                elif (update_type ==  UpdateClassification.MINOR):
                        self.packages['minor'] += package_count
                elif (update_type == UpdateClassification.PATCH):
                        self.packages['patch'] += package_count
                else:
                        self.packages['release'] += package_count

                return allowed_partition