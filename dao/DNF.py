import json
import asyncio

from common.logger import log_timed_execution
from dto.RPM import RPM, RPMUpdate
from dao.Shell import Shell

from common.costants import LIST_UPDATES_CMD
from dto.enums.UpdateClassification import UpdateClassification
from dto.enums.UpdateUrgency import UpdateUrgency

class DNF:
        @log_timed_execution("Getting updates list")
        def get_updates_by_partition_id(self):
                updates: list[dict] = read_updates_list()
                updates_details: list[RPMUpdate] = asyncio.run(get_update_details_from_repository(updates))
                installed_details: dict[str, RPM] = get_installed_details_from_updates(updates_details)
                partition_index = {}

                for update_package in updates_details:
                        update_partition = update_package.get_update_partition()
                        update_urgency = update_package.get_urgency()
                        update_name = update_package.get_package_name()
                        update_version = update_package.get_version()
                        
                        installed_package = installed_details[update_name]
                        installed_version = installed_package.get_version()

                        current_update_type = UpdateClassification.RELEASE

                        if update_partition not in partition_index:
                                partition_index[update_partition] = {
                                        "urgency" : UpdateUrgency.NONE,
                                        "type" : UpdateClassification.MAJOR,
                                        "packages" : []
                                }
                        
                        partition_urgency = partition_index[update_partition]["urgency"]
                        partition_type = partition_index[update_partition]["type"]

                        if update_urgency > partition_urgency:
                                partition_index[update_partition]["urgency"] = update_urgency

                        current_version_str = '.'.join([
                                installed_version.major,
                                installed_version.minor,
                                installed_version.patch
                        ]) + '-' + installed_version.release

                        update_version_str = '.'.join([
                                update_version.major,
                                update_version.minor,
                                update_version.patch
                        ]) + '-' + update_version.release

                        update_details_str = f"{update_name.ljust(60)} {current_version_str} -> {update_version_str}"

                        partition_index[update_partition]["packages"].append(update_details_str)

                        if update_version.major != installed_version.major:
                                current_update_type = UpdateClassification.MAJOR
                        elif update_version.minor != installed_version.minor:
                                current_update_type = UpdateClassification.MINOR
                        elif update_version.patch != installed_version.patch:
                                current_update_type = UpdateClassification.PATCH
                        
                        if current_update_type < partition_type:
                                partition_index[update_partition]["type"] = current_update_type
                                
                return partition_index
                        


def get_installed_details_from_updates(updates_details: list[RPMUpdate]) -> dict:
        installed_details = {}

        for update in updates_details:
                update_package_name = update.get_package_name()
                current_package = RPM.from_package_name(update_package_name)
                installed_details[update_package_name] = current_package
        
        return installed_details


async def get_update_details_from_repository(updates):
        updates_details = await asyncio.gather(*[compose_new_rpm(update) for update in updates])
        return [update for update in updates_details if update]


async def compose_new_rpm(update):
    try:
            current_update = RPMUpdate.from_DNF_output(update)
            return current_update
    except:
            pass


def read_updates_list():
        shell = Shell()
        dnf_output = shell.run(LIST_UPDATES_CMD)
        json_data = json.loads(dnf_output)

        return json_data
