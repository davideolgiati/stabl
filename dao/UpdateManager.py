from common.logger import log_timed_execution
from dao.DNF import DNF
from dto.DNFUpdateEntry import DNFUpdateEntry
from dto.UpdateUrgency import UpdateUrgency
from dto.UpdateClassification import UpdateClassification


class UpdateManager():
        maxAllowedUpgrade = UpdateClassification.PATCH
        maxSkippableUregency = UpdateUrgency.NONE
        packages = {
                "major": 0,
                "minor": 0,
                "patch": 0,
                "release": 0
        }

        def __init__(self, package_manager):
                assert isinstance(package_manager, DNF)
                self.package_manager = package_manager


        def get_updates_list(self):
                self.updates_partitions = self.package_manager.get_updates_by_partition_id()
                assert isinstance(self.updates_partitions, dict)

                return self.updates_partitions
         

        def get_suggested_update_partitions(self):
                assert isinstance(self.updates_partitions, dict)
                assert self.updates_partitions != {}

                suggested_updates = []

                for advisoryId, packagesList in self.updates_partitions.items():
                        assert isinstance(advisoryId, str)
                        assert isinstance(packagesList, list)
                        assert packagesList != []
                        assert advisoryId != ""
                        
                        if(self.evaluate_update_partition(packagesList)):
                                suggested_updates.append(advisoryId)
                
                return suggested_updates
        
        
        def get_majors_count(self):
                return self.packages['major']
        

        def get_minors_count(self):
                return self.packages['minor']
        
        
        def get_patches_count(self):
                return self.packages['patch']
        
        
        def get_releases_count(self):
                return self.packages['release']
        

        def evaluate_update_partition(self, packagesList):
                assert isinstance(packagesList, list)
                assert packagesList != []

                allowedAdvisoryId = True
                securityUpdate = False

                for package in packagesList:
                        assert(isinstance(package, DNFUpdateEntry))
                                
                        if(package.updateUrgency > self.maxSkippableUregency):
                                securityUpdate = True

                        if(package.updateType > self.maxAllowedUpgrade):
                                allowedAdvisoryId = False or securityUpdate

                        if (package.updateType == UpdateClassification.MAJOR):
                                self.packages['major'] += 1
                        elif (package.updateType ==  UpdateClassification.MINOR):
                                self.packages['minor'] += 1
                        elif (package.updateType == UpdateClassification.PATCH):
                                self.packages['patch'] += 1
                        else:
                                self.packages['release'] += 1

                return allowedAdvisoryId