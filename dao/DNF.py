import json
import os
import re
import time

from common.logger import log_timed_execution
from dao.RPM import RPM
from dao.Shell import Shell
from dto.DNFUpdateEntry import DNFUpdateEntry

from common.costants import LIST_UPDATES_CMD, DOWNLOAD_UPGRADE, INSPECT_PKG, GET_SYSTEM_CONFIG
import common.regex as regex
from dao.UpdatesPartitions import UpdatesPartitions

cache_filter = lambda line: line.startswith("cachedir")

class DNF:
        @log_timed_execution("Reading DNF configuration")
        def __init__(self):
                self.shell = Shell()

                dnf_output = self.shell.run(GET_SYSTEM_CONFIG)
                assert isinstance(dnf_output, str)

                dnf_config = dnf_output.split('\n')
                assert len(dnf_config) > 1

                filtered_config = list(filter(cache_filter, dnf_config))
                assert len(filtered_config) == 1

                cache_config = filtered_config[0].split(" = ")
                assert len(cache_config) == 2

                self.cache_dir = cache_config[1]
                assert self.cache_dir is not None

                self.partition_manager = UpdatesPartitions()


        def get_updates_by_partition_id(self):
                updates = self.read_available_update_list()
                assert isinstance(updates, list)

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
                rpm_file = RPM.fromPackagePath(package_path)
                return rpm_file.query_package_info()
        

        def query_installed_package(self, package_name: str):
                installed_package = RPM.fromPackageSignature(package_name)
                return installed_package.query_package_info()