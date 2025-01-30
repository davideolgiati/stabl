import json
import os
import re
import time

from common.logger import Logger
from dao.ShellInterface import ShellInterface
from dto.DNFUpdateEntry import DNFUpdateEntry

from common.costants import LIST_UPDATES_CMD, DOWNLOAD_UPGRADE, INSPECT_PKG, GET_SYSTEM_CONFIG
import common.regex as regex


class DNFHelper:
        def __init__(self):
                self.sh = ShellInterface()
                self.logger = Logger()

                self.logger.info("Reading DNF configuration ... ", end='')
                self.logger.start_timing()

                system_config = self.sh.run(GET_SYSTEM_CONFIG).split('\n')
                filtered_config = [line for line in system_config if line.startswith("cachedir")]

                assert len(filtered_config) == 1

                self.cache_dir = filtered_config[0].split(" = ")[1]

                assert self.cache_dir is not None
                self.logger.stop_timing("done")

        def get_updates_by_partition_id(self):
                self.logger.info("Getting updates partition list ... ", end='')
                self.logger.start_timing()

                assert(LIST_UPDATES_CMD is not None)
                assert(isinstance(LIST_UPDATES_CMD, list))

                raw_json_output = self.sh.run(LIST_UPDATES_CMD)

                assert(isinstance(raw_json_output, str))
                assert(raw_json_output != "")

                packages_list = json.loads(raw_json_output)

                assert(isinstance(packages_list, list))
                self.logger.stop_timing("done")

                updateGruops = {}

                self.logger.info("Parsing updates ... ", end='')
                self.logger.start_timing()
                for package in packages_list:
                        assert(package is not None)
                        assert(isinstance(package, dict))

                        current_package = DNFUpdateEntry(package)
                        if (current_package.key not in updateGruops):
                                updateGruops[current_package.key] = [current_package]
                        else:
                                updateGruops[current_package.key].append(current_package)
                self.logger.stop_timing(f"done")
                
                return updateGruops
        
        def download_updates(self):
                download_updates_cmd = DOWNLOAD_UPGRADE(self.cache_dir)
                assert(isinstance(download_updates_cmd, list))

                self.logger.info("Downloading RPMs from remote ... ", end='')
                self.logger.start_timing()
                self.sh.run(download_updates_cmd)
                self.logger.stop_timing("done")
        
        
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
                shell_output = self.sh.run_unmanaged(inspect_command)
                assert isinstance(shell_output, dict)
                assert isinstance(shell_output.get("code"), int)
                assert isinstance(shell_output.get("info"), str)

                if(shell_output["code"] != 0):
                        return

                rpm_properties = json.loads(shell_output["info"])

                assert isinstance(rpm_properties, dict)
                assert isinstance(rpm_properties.get("Name"), str)
                assert isinstance(rpm_properties.get("Version"), str)
                assert isinstance(rpm_properties.get("Release"), str)
                assert isinstance(rpm_properties.get("Arch"), str)
                assert rpm_properties.get("Name") != ""
                assert rpm_properties.get("Version") != ""
                assert rpm_properties.get("Release") != ""
                assert rpm_properties.get("Arch") != ""

                assert re.findall(regex.package_name, rpm_properties["Name"]) != []

                tokenized_version = re.split(
                        regex.valid_separator, 
                        rpm_properties["Version"]
                )

                if(len(tokenized_version) > 1):
                        rpm_properties["Version"] = tokenized_version[0]
                        additional_info = ''.join(tokenized_version[1:])
                        rpm_properties["Release"] += f"-{additional_info}"


                assert re.findall(regex.package_version, rpm_properties["Version"]) != []        

                return rpm_properties
        

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
        