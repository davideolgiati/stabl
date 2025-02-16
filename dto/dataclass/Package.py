
from common.rpm.properties import query_installed_package_info, query_package_info_from_signature
from dto.dataclass.SemanticVersion import SemanticVersion


class Package():
        _name: str
        _version: SemanticVersion

        @staticmethod
        def from_details(name, version = None):
                assert isinstance(name, str)
                assert name != ""
                
                if not version:
                        version = query_installed_package_info(name)

                result = Package()
                result._name = name
                result._version = version

                return result


        def get_version(self):
                return self._version
        

        def get_name(self):
                return self._name


