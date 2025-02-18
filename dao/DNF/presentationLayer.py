from dao.DNF.applicationLayer import build_installed_index, compose_updates_partitions, get_update_details_form_repository, get_update_list
from model.Package import Package
from model.Update import Update

class DNF:
        def get_update_partitions(self) -> dict:
                update_list: dict = get_update_list()
                updates_index: list[Update] = get_update_details_form_repository(update_list)
                installed_index: dict[str, Package] = build_installed_index(updates_index)
                partitions: dict = compose_updates_partitions(updates_index, installed_index)
                                
                return partitions
