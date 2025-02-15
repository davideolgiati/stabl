
from common.rpm.properties import query_installed_package_info, query_package_info_from_signature
from dto.dataclass.SemanticVersion import SemanticVersion
from dto.enums.UpdateUrgency import UpdateUrgency


class Package():
        _name: str
        _version: SemanticVersion

        @staticmethod
        def from_signature(signature, name = None, version = None):
                assert(isinstance(signature, str))
                assert(signature != "")
        
                if not (name and version):
                        name, version = query_package_info_from_signature(signature)

                assert(isinstance(name, str))
                assert(name != "")

                assert(isinstance(version, SemanticVersion))

                result = Package()
                result._name = name
                result._version = version

                return result
        
        
        @staticmethod
        def from_name(name):
                assert isinstance(name, str)
                assert name != ""
                
                version = query_installed_package_info(name)

                result = Package()
                result._name = name
                result._version = version

                return result


        def get_version(self):
                return self._version
        

        def get_name(self):
                return self._name


class Update(Package):
        _update_urgency: UpdateUrgency
        _partition_id: str
        _package_signature: str

        @staticmethod
        def from_DNF_output(dnf_output):
                
                assert ["name", "version", "release", "arch", "signature", "partition_id", "severity"] == list(dnf_output.keys())
                
                partition_id = dnf_output['partition_id'] 
                package_signature = dnf_output['signature']
                update_urgency = dnf_output['severity'].lower()
                name = dnf_output['name']
                version = SemanticVersion.fromVersionAndRelease(dnf_output['version'], dnf_output['release'])

                result = Package().from_signature(package_signature, name, version)
                result.__class__ = Update
                result._partition_id = partition_id
                result._package_signature = package_signature
                result._update_urgency = UpdateUrgency.fromString(update_urgency)

                return result

        
        def get_signature(self):
                return self._package_signature


        def get_partition(self):
                return self._partition_id


        def get_urgency(self):
                return self._update_urgency