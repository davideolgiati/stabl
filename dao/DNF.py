import json
import os
import re
import time

from common.logger import log_timed_execution
from dao.Shell import Shell
from dto.DNFUpdateEntry import DNFUpdateEntry

from common.costants import LIST_UPDATES_CMD, DOWNLOAD_UPGRADE, INSPECT_PKG, GET_SYSTEM_CONFIG
import common.regex as regex
from dto.UpdatesPartitions import UpdatesPartitions

cache_filter = lambda line: line.startswith("cachedir")

class DNF:
        @log_timed_execution("Reading DNF configuration")
        def __init__(self):
                self.shell = Shell()

                dnf_output = self.shell.run(GET_SYSTEM_CONFIG)
                assert isinstance(dnf_output, str)

                dnf_config = dnf_output.split('\n')
                assert len(dnf_config) > 1

                filtered_config = filter(cache_filter, dnf_config)
                assert len(filtered_config) == 1

                cache_config = filtered_config[0].split(" = ")
                assert len(cache_config) == 2

                self.cache_dir = cache_config[1]
                assert self.cache_dir is not None

                self.partition_manager = UpdatesPartitions()

        @log_timed_execution("Parsing updates")
        def get_updates_by_partition_id(self):
                updates = self.read_available_update_list()
                self.partition_manager.add_packages(updates)
                
                return self.partition_manager.get_partitions()

        @log_timed_execution("Getting updates list")
        def read_available_update_list(self):
            assert(LIST_UPDATES_CMD is not None)
            assert(isinstance(LIST_UPDATES_CMD, list))

            dnf_output = self.shell.run(LIST_UPDATES_CMD)
            assert(isinstance(dnf_output, str))
            assert(dnf_output != "")

            json_data = json.loads(dnf_output)
            assert(isinstance(json_data, list))

            updates = [DNFUpdateEntry(package) for package in json_data]

            return updates
        

        @log_timed_execution("Downloading RPMs from remote")
        def download_updates(self):
                download_updates_cmd = DOWNLOAD_UPGRADE(self.cache_dir)
                assert(isinstance(download_updates_cmd, list))

                self.shell.run(download_updates_cmd)
        

        def query_downloaded_package(self, package_path):
                assert isinstance(package_path, str)
                assert is_valid_rpm_file_path(package_path)
                assert os.path.isfile(package_path)                    
                assert is_file_rpm(package_path) 

                return self.query_package_info(package_path)


        def query_installed_package(self, package_name: str):
                assert(isinstance(package_name, str))
                assert(package_name != "")

                return self.query_package_info(package_name)


        # TODO: https://docs.python.org/3/library/multiprocessing.html#exchanging-objects-between-processes
        def query_package_info(self, package_signature):
                assert(isinstance(package_signature, str))
                assert(package_signature != "")

                inspect_command = INSPECT_PKG(package_signature)
                shell_output = self.shell.run_unmanaged(inspect_command)
                assert isinstance(shell_output, dict)
                assert isinstance(shell_output.get("code"), int)
                assert isinstance(shell_output.get("info"), str)

                if(shell_output["code"] != 0):
                        return

                shell_output["info"] = shell_output["info"].replace("}\n{", "},\n{")
                shell_output["info"] = f"[{shell_output["info"]}]"
                rpms_properties_list = json.loads(shell_output["info"])

                rpm_sort_function = lambda rpm: rpm["Buildtime"]
                rpms_properties_list.sort(key=rpm_sort_function)
                rpm_properties = rpms_properties_list[-1]

                assert isinstance(rpm_properties, dict)
                for key in ["Name", "Version", "Release", "Arch"]:
                        assert isinstance(rpm_properties.get(key), str)
                        assert rpm_properties.get(key) != ""

                assert re.findall(regex.package_name, rpm_properties["Name"]) != []

                self.standardize_rpm_version_structure(rpm_properties)        

                return rpm_properties

        def standardize_rpm_version_structure(self, rpm_properties):
                assert isinstance(rpm_properties, dict)
                assert isinstance(rpm_properties.get("Version"), str)
                assert rpm_properties.get("Version") != ""

                tokenized_version = re.split(
                        regex.valid_separator, 
                        rpm_properties["Version"]
                )

                if(len(tokenized_version) > 1):
                        rpm_properties["Version"] = tokenized_version[0]
                        additional_info = ''.join(tokenized_version[1:])
                        rpm_properties["Release"] += f"-{additional_info}"


                assert re.findall(
                        regex.package_version, 
                        rpm_properties["Version"]
                ) != []
        

def is_valid_rpm_file_path(path):
        assert(isinstance(path, str))

        if re.search(regex.valid_rpm_file, path, re.IGNORECASE):
                return True
        else:
                return False

def is_file_rpm(path):
        assert(isinstance(path, str))
        assert(path != "")

        rpm_magic_bytes = b'\xed\xab\xee\xdb'
        with open(path, 'rb') as fp:
                file_magic_bytes = fp.read(4)

        magic_bytes_check = file_magic_bytes == rpm_magic_bytes
        assert isinstance(magic_bytes_check, bool)

        return magic_bytes_check
        