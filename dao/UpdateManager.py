from common.logger import log_timed_execution
from dao.DNF import DNF
from dto.enums.UpdateUrgency import UpdateUrgency
from dto.enums.UpdateClassification import UpdateClassification


class UpdateManager():
        _max_version_jump = UpdateClassification.PATCH
        _min_uregency = UpdateUrgency.NONE
        _majors = 0
        _minors = 0
        _patches = 0
        _releases = 0
        _partitions = None
        _dnf = None

        def __init__(self, dnf):
                assert isinstance(dnf, DNF)
                self._dnf = dnf


        def get_majors_count(self):
                assert isinstance(self._majors, int)
                return self._majors
        

        def get_minors_count(self):
                assert isinstance(self._minors, int)
                return self._minors
        
        
        def get_patches_count(self):
                assert isinstance(self._patches, int)
                return self._patches
        
        
        def get_releases_count(self):
                assert isinstance(self._releases, int)
                return self._releases
        

        def get_available_partitions(self):
                assert self._dnf is not None
                assert isinstance(self._dnf, DNF)
                assert self._partitions is None

                self._partitions = self._dnf.get_updates_by_partition_id()
                
                assert isinstance(self._partitions, dict)
                return self._partitions


        @log_timed_execution("Evalueting partitions properties")
        def get_suggested_partition_ids(self):
                assert isinstance(self._partitions, dict)
                assert self._partitions != {}

                suggested_updates = []

                for id, partition in self._partitions.items():
                        assert isinstance(id, str)
                        assert isinstance(partition, dict)
                        assert id != ""
                        assert partition != {}

                        self.compute_update_statistics(partition)
                        
                        if(self.evaluate_partition_proprties(partition)):
                                suggested_updates.append(id)
                

                assert isinstance(suggested_updates, list)
                return suggested_updates

        def compute_update_statistics(self, partition):
                assert isinstance(partition, dict) 
                assert "packages" in partition.keys()
                assert "type" in partition.keys()
                assert isinstance(partition["packages"], list)
                assert isinstance(partition["type"], UpdateClassification)

                package_count = len(partition["packages"])
                update_type = partition["type"]

                if (update_type == UpdateClassification.MAJOR):
                        assert isinstance(self._majors, int)
                        self._majors += package_count
                elif (update_type ==  UpdateClassification.MINOR):
                        assert isinstance(self._minors, int)
                        self._minors += package_count
                elif (update_type == UpdateClassification.PATCH):
                        assert isinstance(self._patches, int)
                        self._patches += package_count
                else:
                        assert isinstance(self._releases, int)
                        self._releases += package_count
        

        def evaluate_partition_proprties(self, partition):
                assert isinstance(partition, dict)
                assert "urgency" in partition.keys()
                assert "type" in partition.keys()
                assert isinstance(partition["urgency"], UpdateUrgency)
                assert isinstance(partition["type"], UpdateClassification)

                urgency = partition["urgency"]
                type = partition["type"]
                is_valid = False

                if(urgency > self._min_uregency):
                        is_valid = True

                if(type <= self._max_version_jump):
                        is_valid = True


                assert isinstance(is_valid, bool)
                return is_valid