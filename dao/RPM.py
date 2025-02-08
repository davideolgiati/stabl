
from common.rpm.properties import query_installed_package_info, query_package_info_from_signature
from dto.dataclass.SemanticVersion import SemanticVersion


class RPM():
        package_reference: str
        package_version: SemanticVersion

        @staticmethod
        def from_package_signature(package_signature):
                assert(isinstance(package_signature, str))
                assert(package_signature != "")
        
                version_info = query_package_info_from_signature(package_signature)

                new_obj = RPM()
                new_obj.package_reference = package_signature
                new_obj.package_version = version_info

                return new_obj
        
        
        @staticmethod
        def from_package_name(package_name):
                assert isinstance(package_name, str)
                assert package_name != ""
                
                version_info = query_installed_package_info(package_name)

                new_obj = RPM()
                new_obj.package_reference = package_name
                new_obj.package_version = version_info

                return new_obj

        def get_version(self):
                return self.package_version
        
        def get_package_reference(self):
                return self.package_reference

        