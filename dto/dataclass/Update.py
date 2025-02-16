from dto.dataclass.Package import Package
from dto.dataclass.SemanticVersion import SemanticVersion
from dto.enums.UpdateUrgency import UpdateUrgency


class Update(Package):
        _urgency: UpdateUrgency
        _partition_id: str
        _signature: str

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
                assert query['arch']         != '' # Future use
                assert query['signature']    != ''
                assert query['partition_id'] != ''
                assert query['severity']     != ''
                
                partition_id = query['partition_id'] 
                
                signature = query['signature']
                name = query['name']
                
                version = query['version']
                release = query['release']

                urgency = query['severity']
                semantic_version = SemanticVersion.fromVersionAndRelease(
                        version, release
                )

                result = Package().from_details(name, semantic_version)
                result.__class__ = Update
                result._partition_id = partition_id
                result._signature = signature
                result._urgency = UpdateUrgency.fromString(urgency)

                assert isinstance(result._partition_id, str)
                assert isinstance(result._signature, str)
                assert isinstance(result._urgency, UpdateUrgency)
                assert isinstance(result._name, str)
                assert isinstance(result._version, SemanticVersion)

                assert result._partition_id != ''
                assert result._signature != ''
                assert result._name != ''

                return result

        
        def get_signature(self):
                return self._signature


        def get_partition(self):
                return self._partition_id


        def get_urgency(self):
                return self._urgency