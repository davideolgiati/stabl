from common.logger import log_timed_execution
from dao.DNF.systemLayer import get_update_list_from_repository, query_packages_repository
from model.Package import Package
from model.Update import Update
from model.enum.SecurityClass import SecurityClass
from model.enum.UpdateClass import UpdateClass


@log_timed_execution('Getting updates partition list')
def get_update_list() -> dict:
        json_data: list[dict] = get_update_list_from_repository()
        updates_index: dict = build_update_index(json_data)

        assert isinstance(updates_index, dict)

        return updates_index


@log_timed_execution('Getting installed packages details')
def build_installed_index(update_packages: list[Update]) -> dict:
        assert isinstance(update_packages, list)

        installed_packages = {}

        for update in update_packages:
                assert isinstance(update, Update)
                
                update_name = update.get_name()

                assert isinstance(update_name, str)
                assert update_name != ''

                installed = Package.from_details(update_name)
                
                assert installed != None
                assert installed != Package()
                assert installed.get_name() == update_name
                
                installed_packages[update_name] = installed
        
        return installed_packages


@log_timed_execution('Getting updates packages details')
def build_update_index(updates):
        assert isinstance(updates, dict)

        updates_signature_list: list[str] = list(updates.keys())
        packages_details_from_repo: list[dict] = query_packages_repository(updates_signature_list)

        for update in packages_details_from_repo:
                keys = update['signature']
                key = None

                if keys[0] in updates_signature_list:
                        key = keys[0]
                elif keys[1] in updates_signature_list:
                        key = keys[1]
                
                if key is not None:
                        update['signature'] = key
                        partition_details = updates[key]
                        partition_id = partition_details['partition_id']
                        severity = partition_details['severity']

                        update['partition_id'] = partition_id
                        update['severity'] = severity

                        assert ['name', 'version', 'release', 'arch', 'signature', 'partition_id', 'severity'] == list(update.keys())
                else:
                        print(f'{key} is missing!')

        updates_details = [Update.from_repository_query(update) for update in packages_details_from_repo]
        return [update for update in updates_details if update]


def compose_updates_partitions(updates_index, installed_index):
        partition_index = {}

        for update in updates_index:
                partition = update.get_partition()
                update_urgency = update.get_urgency()
                update_name = update.get_name()
                update_version = update.get_version()
                        
                installed = installed_index[update_name]
                installed_version = installed.get_version()

                current_update_type = UpdateClass.RELEASE

                if partition not in partition_index:
                        partition_index[partition] = {
                                        'urgency' : SecurityClass.NONE,
                                        'type' : UpdateClass.MAJOR,
                                        'packages' : []
                                }
                        
                partition_urgency = partition_index[partition]['urgency']
                partition_type = partition_index[partition]['type']

                if update_urgency > partition_urgency:
                        partition_index[partition]['urgency'] = update_urgency

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

                update_details_str = f'{update_name.ljust(60)} {current_version_str} -> {update_version_str}'

                partition_index[partition]['packages'].append(update_details_str)

                if update_version.major != installed_version.major:
                        current_update_type = UpdateClass.MAJOR
                elif update_version.minor != installed_version.minor:
                        current_update_type = UpdateClass.MINOR
                elif update_version.patch != installed_version.patch:
                        current_update_type = UpdateClass.PATCH
                        
                if current_update_type < partition_type:
                        partition_index[partition]['type'] = current_update_type

        return partition_index


def build_update_index(json_data: list[dict]) -> dict:
        assert isinstance(json_data, list)
        updates_index: dict = {}

        for update in json_data:
                key, value = build_update_entry(update)

                assert isinstance(key, str)
                assert isinstance(value, dict)
                assert ['partition_id', 'severity'] == list(value.keys())
                
                assert isinstance(value['severity'], str)
                assert isinstance(value['partition_id'], str)

                assert value['severity'] != ''
                assert value['partition_id'] != ''

                updates_index[key] = value
                
        assert len(updates_index.keys()) == len(json_data)

        return updates_index


def build_update_entry(update: dict) -> str | dict:
        assert ['name', 'type', 'severity', 'nevra', 'buildtime'] == list(update.keys())

        assert isinstance(update['name'], str)
        assert isinstance(update['nevra'], str)
        assert isinstance(update['severity'], str)

        assert update['name'] != ''
        assert update['nevra'] != ''
        assert update['severity'] != ''

        key: str = update['nevra']
        partition_id: str = update['name']
        severity: str  = update['severity']

        value: dict = {
                'partition_id': partition_id, 
                'severity': severity
        }

        return key, value
