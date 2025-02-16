from dao.UpdateManager import UpdateManager
from dao.DNF import DNF

def display_message_to_user(suggested_update_partitions):
        print()
        if(suggested_update_partitions != []):
                advisories_string = ','.join(suggested_update_partitions)
                print(f"suggested updates: sudo dnf update --advisory={advisories_string}")
        else:
                print(f"no suggested updates found") 
        print()

def display_partition_id_details(partition_properties, partition_id):
        stdout_buffer = ""
        packages_list = partition_properties["packages"]


        for package in packages_list:
                stdout_buffer += f"\t{package}\n"
                
        print(f"Advisory Id: \"{partition_id}\"")
        print(f"{stdout_buffer}")

def display_update_summary(update_manager):
        print()
        print()
        print(f"Major updates   : {update_manager.get_majors_count()}")
        print(f"Minor updates   : {update_manager.get_minors_count()}")
        print(f"Patch updates   : {update_manager.get_patches_count()}")
        print(f"Release updates : {update_manager.get_releases_count()}")
        print()

def setup_package_mamager():
        package_manager = DNF()
        update_manager = UpdateManager(package_manager)
        return update_manager

def main():
        update_manager = setup_package_mamager()

        updates_list = update_manager.get_available_partitions()
        suggested_updates_partitions = update_manager.get_suggested_partition_ids()

        display_update_summary(update_manager)
        
        for partition_id in suggested_updates_partitions:
                partition_properties = updates_list[partition_id]
                display_partition_id_details(partition_properties, partition_id)

        display_message_to_user(suggested_updates_partitions)


if __name__ == "__main__":
        main()