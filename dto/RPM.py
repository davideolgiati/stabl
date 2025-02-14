
from common.rpm.properties import query_installed_package_info, query_package_info_from_signature
from dto.dataclass.SemanticVersion import SemanticVersion
from dto.enums.UpdateUrgency import UpdateUrgency


class RPM():
        _package_name: str
        _package_version: SemanticVersion

        @staticmethod
        def from_package_signature(package_signature, name = None, version = None):
                assert(isinstance(package_signature, str))
                assert(package_signature != "")
        
                if not (name and version):
                        name, version = query_package_info_from_signature(package_signature)

                new_obj = RPM()
                new_obj._package_name = name
                new_obj._package_version = version

                return new_obj
        
        
        @staticmethod
        def from_package_name(package_name):
                assert isinstance(package_name, str)
                assert package_name != ""
                
                version_info = query_installed_package_info(package_name)

                new_obj = RPM()
                new_obj._package_name = package_name
                new_obj._package_version = version_info

                return new_obj

        def get_version(self):
                return self._package_version
        
        def get_package_name(self):
                return self._package_name


class RPMUpdate(RPM):
        _update_urgency: UpdateUrgency
        _partition_id: str
        _package_signature: str

        @staticmethod
        def from_DNF_output(dnf_output):
                assert ["name", "version", "release", "arch", "signature", "partition_id", "severity"] == dnf_output.keys()
                partition_id = dnf_output['partition_id'] 
                package_signature = dnf_output['signature']
                update_urgency = dnf_output['severity'].lower()
                name = dnf_output['name']
                version = dnf_output['version']

                new_obj = RPM().from_package_signature(package_signature, name, version)
                new_obj.__class__ = RPMUpdate
                new_obj._partition_id = partition_id
                new_obj._package_signature = package_signature
                new_obj._update_urgency = UpdateUrgency.fromString(update_urgency)

                return new_obj
        
        @staticmethod
        def from_DNF_output_list(dnf_output):
                partition_id = dnf_output['name'] 
                package_signature = dnf_output['nevra']
                update_urgency = dnf_output['severity'].lower()

                new_obj = RPM().from_package_signature(package_signature)
                new_obj.__class__ = RPMUpdate
                new_obj._partition_id = partition_id
                new_obj._package_signature = package_signature
                new_obj._update_urgency = UpdateUrgency.fromString(update_urgency)

                return new_obj
        
        def get_package_signature(self):
                return self._package_signature
        
        def get_update_partition(self):
                return self._partition_id
        
        def get_urgency(self):
                return self._update_urgency