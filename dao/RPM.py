
from common.rpm.properties import format_package_version, process_repoquery_output, process_rpm_json_output, run_dnf_repoquery_command, run_rpm_query_command, unpack_version_string
from dao.Shell import Shell


class RPM():
        def __init__(self, package_name=None, package_signature=None):
                assert package_name is not None or package_signature is not None

                self.shell = Shell()

                if package_name is not None:
                        self.package_reference = package_name
                        self.installed = True
                elif package_signature is not None:
                        self.package_reference = package_signature
                        self.installed = False

        @staticmethod
        def fromPackageSignature(package_signature):
                assert(isinstance(package_signature, str))
                assert(package_signature != "")
        
                return RPM(package_signature = package_signature)
        
        @staticmethod
        def fromPackageName(package_name):
                assert isinstance(package_name, str)
                assert package_name != ""
                
                return RPM(package_name = package_name)

        def query_package_info(self):
                assert(isinstance(self.package_reference, str))
                assert(self.package_reference != "")

                if self.installed:
                        stdout_message = run_rpm_query_command(self.package_reference)
                        rpm_properties = process_rpm_json_output(stdout_message)
                else:
                        stdout_message = run_dnf_repoquery_command(self.package_reference)
                        if stdout_message == '':
                                raise KeyError
                        rpm_properties = process_repoquery_output(stdout_message)

                rpm_version = rpm_properties["Version"]
                rpm_release = rpm_properties["Release"]

                final_version = format_package_version(rpm_version, rpm_release)
                assert isinstance(final_version, dict)
                assert isinstance(final_version.get("version"), str)
                assert isinstance(final_version.get("release"), str)

                rpm_properties["Version"] = final_version["version"]
                rpm_properties["Release"] = final_version["release"] 

                return rpm_properties


        