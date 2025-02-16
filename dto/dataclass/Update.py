from dto.dataclass.Package import Package
from dto.dataclass.SemanticVersion import SemanticVersion
from dto.enums.UpdateUrgency import UpdateUrgency


class Update(Package):
        _update_urgency: UpdateUrgency
        _partition_id: str
        _package_signature: str

        @staticmethod
        def from_repository_query(query):
                
                assert "name"         in query.keys()
                assert "version"      in query.keys()
                assert "release"      in query.keys()
                assert "arch"         in query.keys()
                assert "signature"    in query.keys()
                assert "partition_id" in query.keys()
                assert "severity"     in query.keys()
                assert len(query.keys()) == 7

                assert isinstance(query['name'], str)
                assert isinstance(query['version'], str)
                assert isinstance(query['release'], str)
                assert isinstance(query['arch'], str)
                assert isinstance(query['signature'], str)
                assert isinstance(query['partition_id'], str)
                assert isinstance(query['severity'], str)
                
                assert query['name']         != ''
                assert query['version']      != ''             
                assert query['release']      != ''
                assert query['arch']         != ''
                assert query['signature']    != ''
                assert query['partition_id'] != ''
                assert query['severity']     != ''
                
                partition_id = query['partition_id'] 
                package_signature = query['signature']
                update_urgency = query['severity'].lower()
                name = query['name']
                version = SemanticVersion.fromVersionAndRelease(query['version'], query['release'])

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