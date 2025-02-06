import json
import os
import re
import time

from common.logger import log_timed_execution
from dao.RPM import RPM
from dao.Shell import Shell
from dto.DNFUpdateEntry import DNFUpdateEntry

from common.costants import LIST_UPDATES_CMD, GET_SYSTEM_CONFIG
import common.regex as regex
from dao.UpdatesPartitions import UpdatesPartitions

class DNF:
        @log_timed_execution("Reading DNF configuration")
        def __init__(self):
                self.shell = Shell()

                dnf_output = self.shell.run(GET_SYSTEM_CONFIG)
                assert isinstance(dnf_output, str)

                dnf_config = dnf_output.split('\n')
                assert len(dnf_config) > 1

                self.partition_manager = UpdatesPartitions()

        @log_timed_execution("Getting updates list")
        def get_updates_by_partition_id(self):
                updates = self.read_available_update_list()
                assert isinstance(updates, list)

                self.partition_manager.add_packages(updates)

                return self.partition_manager.get_partitions()
        
        def read_available_update_list(self):
                assert(LIST_UPDATES_CMD is not None)
                assert(isinstance(LIST_UPDATES_CMD, list))

                dnf_output = self.shell.run(LIST_UPDATES_CMD)
                assert(isinstance(dnf_output, str))
                assert(dnf_output != "")

                json_data = json.loads(dnf_output)
                assert(isinstance(json_data, list))

                updates = [DNFUpdateEntry(package) for package in json_data]
                updates_rpms = {}

                problematic_entries = []

                for update in updates:
                        try:
                                rpm_info = RPM.fromPackageSignature(update.packageName)
                                updates_rpms[update.packageName] = rpm_info.query_package_info()
                        except KeyError: #TODO: eccezione specifica
                                problematic_entries.append(update)
                                pass
                
                for update in problematic_entries:
                        updates.remove(update)

                installed_rpms = {id: RPM.fromPackageName(update["Name"]).query_package_info() for id, update in updates_rpms.items()}

                final_updates = []

                for update in updates:
                        update_info = updates_rpms[update.packageName]
                        installed_info = installed_rpms[update.packageName]

                        update.set_new_version(update_info)
                        update.set_current_version(installed_info)

                        update.compute_update_type()

                        final_updates.append(update)

                return final_updates