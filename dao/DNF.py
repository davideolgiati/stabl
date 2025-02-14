import json
import asyncio

from common.logger import log_timed_execution
from dto.RPM import RPM, RPMUpdate
from dao.Shell import Shell

from common.costants import GET_UPDATE_DETAILS, LIST_UPDATES_CMD
from dto.enums.UpdateClassification import UpdateClassification
from dto.enums.UpdateUrgency import UpdateUrgency

class DNF:
        @log_timed_execution("Getting updates list")
        def get_updates_by_partition_id(self):
                updates: dict = read_updates_list()

                assert isinstance(updates, dict)
                assert all([isinstance(update, dict) for _, update in updates.items()])
                assert all([['partition_id', 'severity'] == list(update.keys()) for _, update in updates.items()])

                updates_details: list[RPMUpdate] = get_update_details_from_repository(updates)

                
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


def get_update_details_from_repository(updates):
        updates_signature_list: list[str] = list(updates.keys())
        packages_details_from_repo: list[dict] = query_remote_repo_for_details(updates_signature_list)

        for update in packages_details_from_repo:
                key = update["signature"]
                partition_details = updates[key]
                partition_id = partition_details['partition_id']
                severity = partition_details['severity']

                update["partition_id"] = partition_id
                update["severity"] = severity

                assert ["name", "version", "release", "arch", "signature", "partition_id", "severity"] == list(update.keys())

        updates_details = [RPMUpdate.from_DNF_output(update) for update in packages_details_from_repo]
        return [update for update in updates_details if update]


def query_remote_repo_for_details(update_signature_list) -> list[dict]:
        assert all([isinstance(pkg, str) for pkg in update_signature_list])
        shell: Shell = Shell()
        repo_query_cmd: list[str] = GET_UPDATE_DETAILS(update_signature_list)

        assert all([isinstance(token, str) for token in repo_query_cmd])

        repo_query_output: str = shell.run(repo_query_cmd)
        valid_json_repoquery_output: str = f"[{repo_query_output}]"

        parsed_json_repoquery_output: list[dict] = json.loads(valid_json_repoquery_output)

        assert all([["name", "version", "release", "arch", "signature"] == list(pkg.keys()) for pkg in parsed_json_repoquery_output])

        return parsed_json_repoquery_output


def read_updates_list() -> dict:
        json_data: list[dict] = get_dnf_updatelist_output()
        updates_index: dict = compose_update_index_dictionary(json_data)

        return updates_index

def compose_update_index_dictionary(json_data: list[dict]) -> dict:
        assert isinstance(json_data, list)
        assert all(["nevra" in update.keys() for update in json_data])
        assert all(["name" in update.keys() for update in json_data])
        assert all(["severity" in update.keys() for update in json_data])

        updates_index: dict = {}

        for update in json_data:
                key: str = update["nevra"]
                severity: str  = update["severity"]
                partition_id: str = update["name"]

                updates_index[key] = {
                        'partition_id': partition_id, 
                        'severity': severity
                }
                
        return updates_index

def get_dnf_updatelist_output() -> dict:
        shell: Shell = Shell()
        dnf_output: str = shell.run(LIST_UPDATES_CMD)

        assert isinstance(dnf_output, str)
        assert dnf_output != ""

        json_data: list[dict] = json.loads(dnf_output)
        return json_data
