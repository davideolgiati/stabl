import json
import asyncio

from common.logger import log_timed_execution
from dto.dataclass.Package import Package
from dto.dataclass.Update import Update
from dao.Shell import Shell

from common.costants import GET_UPDATE_DETAILS, LIST_UPDATES_CMD
from dto.enum.UpdateClass import UpdateClass
from dto.enum.SecurityClass import SecurityClass

class DNF:
        def get_update_partitions(self):
                updates: dict = get_updates()

                assert isinstance(updates, dict)
                assert all([isinstance(update, dict) for _, update in updates.items()])
                assert all([['partition_id', 'severity'] == list(update.keys()) for _, update in updates.items()])

                updates_details: list[Update] = get_update_details(updates)

                installed_details: dict[str, Package] = get_installed_details(updates_details)
                
                partition_index = self.group_updates_by_partitions(updates_details, installed_details)
                                
                return partition_index

        def group_updates_by_partitions(self, updates_details, installed_details):
                partition_index = {}

                for update_package in updates_details:
                        update_partition = update_package.get_partition()
                        update_urgency = update_package.get_urgency()
                        update_name = update_package.get_name()
                        update_version = update_package.get_version()
                                
                        installed_package = installed_details[update_name]
                        installed_version = installed_package.get_version()

                        current_update_type = UpdateClass.RELEASE

                        if update_partition not in partition_index:
                                partition_index[update_partition] = {
                                                "urgency" : SecurityClass.NONE,
                                                "type" : UpdateClass.MAJOR,
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
                                current_update_type = UpdateClass.MAJOR
                        elif update_version.minor != installed_version.minor:
                                current_update_type = UpdateClass.MINOR
                        elif update_version.patch != installed_version.patch:
                                current_update_type = UpdateClass.PATCH
                                
                        if current_update_type < partition_type:
                                partition_index[update_partition]["type"] = current_update_type

                return partition_index
                        

@log_timed_execution("Getting installed packages details")
def get_installed_details(updates_details: list[Update]) -> dict:
        installed_details = {}

        for update in updates_details:
                update_package_name = update.get_name()
                current_package = Package.from_details(update_package_name)
                installed_details[update_package_name] = current_package
        
        return installed_details

@log_timed_execution("Getting updates details")
def get_update_details(updates):
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

        updates_details = [Update.from_repository_query(update) for update in packages_details_from_repo]
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
def get_updates() -> dict:
        json_data: list[dict] = get_dnf_updatelist()

        assert isinstance(json_data, list)
        assert all([isinstance(entry, dict) for entry in json_data])

        assert all(["name"      in entry.keys() for entry in json_data])
        assert all(["type"      in entry.keys() for entry in json_data])
        assert all(["nevra"     in entry.keys() for entry in json_data])
        assert all(["severity"  in entry.keys() for entry in json_data])
        assert all(["buildtime" in entry.keys() for entry in json_data])

        assert all([entry["name"] != ''      for entry in json_data])
        assert all([entry["type"] != ''      for entry in json_data])
        assert all([entry["nevra"] != ''     for entry in json_data])
        assert all([entry["severity"] != ''  for entry in json_data])
        assert all([entry["buildtime"] != '' for entry in json_data])

        assert all([len(entry.keys()) == 5 for entry in json_data])

        updates_index: dict = build_update_index(json_data)

        assert all([isinstance(key, str) and isinstance(value, dict) for key, value in updates_index.items()])
        assert all(["partition_id" in value.keys() for _, value in updates_index.items()])
        assert all(["severity" in value.keys()     for _, value in updates_index.items()])
        assert all([isinstance(value["partition_id"], str) for _, value in updates_index.items()])
        assert all([isinstance(value["severity"], str)     for _, value in updates_index.items()])
        assert all([value["partition_id"] != '' for _, value in updates_index.items()])
        assert all([value["severity"] != ''     for _, value in updates_index.items()])

        return updates_index

def build_update_index(json_data: list[dict]) -> dict:
        assert isinstance(json_data, list)
        updates_index: dict = {}

        for update in json_data:
                key, value = compose_new_index_entry(update)
                updates_index[key] = value
                
        assert len(updates_index.keys()) == len(json_data)

        return updates_index

def compose_new_index_entry(update):
        assert "name" in update.keys()
        assert "nevra" in update.keys()
        assert "severity" in update.keys()

        assert isinstance(update['name'], str)
        assert isinstance(update['nevra'], str)
        assert isinstance(update['severity'], str)

        assert update['name'] != ''
        assert update['nevra'] != ''
        assert update['severity'] != ''

        key: str = update["nevra"]
        partition_id: str = update["name"]
        severity: str  = update["severity"]

        value = {
                'partition_id': partition_id, 
                'severity': severity
        }

        return key, value

def get_dnf_updatelist() -> dict:
        shell: Shell = Shell()
        dnf_output: str = shell.run(LIST_UPDATES_CMD)

        assert isinstance(dnf_output, str)
        assert dnf_output != ""

        json_data: list[dict] = json.loads(dnf_output)
        return json_data
