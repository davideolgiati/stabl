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

def display_advisory_id_details(current_packages_list, advisoryId):
        stdout_buffer = ""

        for package in current_packages_list:
                stdout_buffer += f"\t{package.packageName.ljust(60)}\n"
                
        print(f"Advisory Id: \"{advisoryId}\"")
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

        updates_list = update_manager.get_updates_list()
        suggested_updates_partitions = update_manager.get_suggested_update_partitions()

        display_update_summary(update_manager)
        
        for partition_id in suggested_updates_partitions:
                current_packages_list = updates_list[partition_id]
                display_advisory_id_details(current_packages_list, partition_id)

        display_message_to_user(suggested_updates_partitions)


if __name__ == "__main__":
        main()