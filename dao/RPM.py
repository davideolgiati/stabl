import json
import os
import re
from common import regex
from common.costants import INSPECT_PKG
from common.rpm.files import is_file_rpm, is_valid_rpm_file_path
from common.rpm.properties import format_package_version, process_rpm_json_output, run_rpm_query_command
from dao.Shell import Shell


class RPM():
        def __init__(self, package_path=None, package_signature=None):
                assert package_path is not None or package_signature is not None

                self.shell = Shell()

                if package_path is not None:
                        self.package_reference = package_path
                elif package_signature is not None:
                        self.package_reference = package_signature

        @staticmethod
        def fromPackageSignature(package_signature):
                assert(isinstance(package_signature, str))
                assert(package_signature != "")
        
                return RPM(package_signature = package_signature)
        
        @staticmethod
        def fromPackagePath(package_path):
                assert isinstance(package_path, str)
                assert is_valid_rpm_file_path(package_path)
                assert os.path.isfile(package_path)                    
                assert is_file_rpm(package_path)
                
                return RPM(package_path = package_path)

        def query_package_info(self):
                assert(isinstance(self.package_reference, str))
                assert(self.package_reference != "")

                try:
                        stdout_message = run_rpm_query_command(self.package_reference)
                except ValueError:
                        return

                rpm_properties = process_rpm_json_output(stdout_message)

                rpm_version = rpm_properties["Version"]
                rpm_release = rpm_properties["Release"]

                final_version = format_package_version(rpm_version, rpm_release)
                assert isinstance(final_version, dict)
                assert isinstance(final_version.get("version"), str)
                assert isinstance(final_version.get("release"), str)

                rpm_properties["Version"] = final_version["version"]
                rpm_properties["Release"] = final_version["release"]        

                return rpm_properties


        