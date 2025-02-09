import json
import concurrent.futures

from common.logger import log_timed_execution
from dto.RPM import RPM, RPMUpdate
from dao.Shell import Shell

from common.costants import LIST_UPDATES_CMD, GET_SYSTEM_CONFIG
from dao.UpdatesPartitions import UpdatesPartitions
from dto.enums.UpdateClassification import UpdateClassification
from dto.enums.UpdateUrgency import UpdateUrgency

class DNF:
        @log_timed_execution("Reading DNF configuration")
        def __init__(self):
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
                updates: list[dict] = read_updates_list()
                updates_details: list[RPMUpdate] = get_update_details_from_repository(updates)
                installed_details: dict = get_installed_details_from_updates(updates_details)
                partition_index = {}

                for update in updates_details:
                        current_partition = update.get_update_partition()
                        current_urgency = update.get_urgency()
                        current_package = update.get_package_name()

                        if current_partition not in partition_index:
                                partition_index[current_partition] = {
                                        "urgency" : UpdateUrgency.NONE,
                                        "type" : UpdateClassification.MAJOR,
                                        "packages" : {}
                                }
                        
                        partition_urgency = partition_index[current_partition]["urgency"]

                        if current_urgency > partition_urgency:
                                partition_urgency = current_urgency

                        partition_index[current_partition]["packages"][current_package] = {
                                "installed" : installed_details[current_package],
                                "update" : update
                        }

                        # TODO: confronto versioni




def get_installed_details_from_updates(updates_details: list[RPMUpdate]) -> dict:
        installed_details = {}

        for update in updates_details:
                update_package_name = update.get_package_name()
                current_package = RPM.from_package_name(update_package_name)
                installed_details[update_package_name] = current_package
        
        return installed_details


def get_update_details_from_repository(updates):
        updates_details = []
        for update in updates:
                try:
                        current_update = RPMUpdate.from_DNF_output(update)
                        updates_details.append(current_update)
                except:
                        pass


def read_updates_list():
        shell = Shell()
        dnf_output = shell.run(LIST_UPDATES_CMD)
        json_data = json.loads(dnf_output)

        return json_data
