from dto.dataclass.Package import Package
from dto.dataclass.SemanticVersion import SemanticVersion
from dto.enums.UpdateUrgency import UpdateUrgency


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