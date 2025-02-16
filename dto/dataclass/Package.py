
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


