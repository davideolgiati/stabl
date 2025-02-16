import json
import asyncio

from common.logger import log_timed_execution
from dto.dataclass.Package import Package, Update
from dao.Shell import Shell

from common.costants import GET_UPDATE_DETAILS, LIST_UPDATES_CMD
from dto.enums.UpdateClassification import UpdateClassification
from dto.enums.UpdateUrgency import UpdateUrgency

class DNF:
        def get_updates_by_partition_id(self):
                updates: dict = read_updates_list()

                assert isinstance(updates, dict)
                assert all([isinstance(update, dict) for _, update in updates.items()])
                assert all([['partition_id', 'severity'] == list(update.keys()) for _, update in updates.items()])

                updates_details: list[Update] = get_update_details_from_repository(updates)

                
                installed_details: dict[str, Package] = get_installed_details_from_updates(updates_details)
                partition_index = {}

                for update_package in updates_details:
                        update_partition = update_package.get_partition()
                        update_urgency = update_package.get_urgency()
                        update_name = update_package.get_name()
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
                        

@log_timed_execution("Getting installed packages details")
def get_installed_details_from_updates(updates_details: list[Update]) -> dict:
        installed_details = {}

        for update in updates_details:
                update_package_name = update.get_name()
                current_package = Package.from_name(update_package_name)
                installed_details[update_package_name] = current_package
        
        return installed_details

@log_timed_execution("Getting updates details")
def get_update_details_from_repository(updates):
        updates_signature_list: list[str] = list(updates.keys())
        packages_details_from_repo: list[dict] = query_packages_repository(updates_signature_list)

        for update in packages_details_from_repo:
                keys = update["signature"]
                key = None

                if keys[0] in updates_signature_list:
                        key = keys[0]
                elif keys[1] in updates_signature_list:
                        key = keys[1]
                
                if key is not None:
                        update["signature"] = key
                        partition_details = updates[key]
                        partition_id = partition_details['partition_id']
                        severity = partition_details['severity']

                        update["partition_id"] = partition_id
                        update["severity"] = severity

                        assert ["name", "version", "release", "arch", "signature", "partition_id", "severity"] == list(update.keys())
                else:
                        print(f"{key} is missing!")

        updates_details = [Update.from_DNF_output(update) for update in packages_details_from_repo]
        return [update for update in updates_details if update]


def query_packages_repository(signatures) -> list[dict]:
        assert isinstance(signatures, list)
        assert all([ isinstance(signature, str) for signature in signatures ])
        
        if len(signatures) == 0:
                return []

        assert len(signatures) > 0        

        shell: Shell = Shell()
        repository_query: list[str] = GET_UPDATE_DETAILS(signatures)

        assert isinstance(repository_query, list)
        assert repository_query != []
        assert all([isinstance(token, str) for token in repository_query])

        query_result: str = shell.run(repository_query)
        parsed_result = parse_query_result(query_result)

        return parsed_result

def parse_query_result(query_result):
    assert isinstance(query_result, str)

    json_result: str = f"[{query_result[:-1]}]"

    assert isinstance(json_result, str)
    assert json_result != query_result
    assert json_result != ""

    parsed_result: list[dict] = json.loads(json_result)

    assert isinstance(parsed_result, list)
    for package in parsed_result:
            assert isinstance(package, dict)
            assert len(package.keys()) == 5
                
            assert "name"      in package.keys()
            assert "version"   in package.keys()
            assert "release"   in package.keys()
            assert "arch"      in package.keys()
            assert "signature" in package.keys()
                
            assert isinstance(package["name"], str)
            assert isinstance(package["version"], str)
            assert isinstance(package["release"], str)
            assert isinstance(package["arch"], str)
            assert isinstance(package["signature"], list)
            assert len(package["signature"]) == 2
            assert isinstance(package["signature"][0], str)
            assert isinstance(package["signature"][1], str)

            assert package["name"] != ''
            assert package["version"] != ''
            assert package["release"] != ''
            assert package["arch"] != ''
            assert package["signature"][0] != ''
            assert package["signature"][1] != ''
            
    return parsed_result

@log_timed_execution("Getting updates partition list")
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
