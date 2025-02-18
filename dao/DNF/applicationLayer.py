from common.logger import log_timed_execution
from dao.DNF.systemLayer import get_update_list_from_repository
from model.Package import Package
from model.Update import Update


@log_timed_execution('Getting updates partition list')
def get_update_list() -> dict:
        json_data: list[dict] = get_update_list_from_repository()
        updates_index: dict = build_update_index(json_data)

        assert isinstance(updates_index, dict)

        return updates_index


def build_update_index(json_data: list[dict]) -> dict:
        assert isinstance(json_data, list)
        updates_index: dict = {}

        for update in json_data:
                key, value = build_index_entry(update)

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


def build_index_entry(update: dict) -> str | dict:
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

@log_timed_execution('Getting installed packages details')
def get_installed_details(updates_details: list[Update]) -> dict:
        installed_details = {}

        for update in updates_details:
                update_package_name = update.get_name()
                current_package = Package.from_details(update_package_name)
                installed_details[update_package_name] = current_package
        
        return installed_details