from dao.UpdateManager import UpdateManager
from dao.DNF import DNF

def display_message_to_user(suggestedAdvisoryIds):
    print()
    if(suggestedAdvisoryIds != []):
            advisories_string = ','.join(suggestedAdvisoryIds)
            print(f"suggested updates: sudo dnf update --advisory={advisories_string}")
    else:
            print(f"no suggested updates found") 
    print()

def print_advisory_id_details(current_packages_list, advisoryId):
        printBuffer = ""

        for package in current_packages_list:
                printBuffer += f"\t{package.packageName.ljust(60)}\n"
                
        print(f"Advisory Id: \"{advisoryId}\" \n{printBuffer}")

def setup_package_mamager():
        packageManager = DNF()
        updateManager = UpdateManager(packageManager)
        return updateManager

def main():
        updateManager = setup_package_mamager()

        packagesByAdvisoryId = updateManager.get_updates_by_advisory_id()
        suggestedAdvisoryIds = updateManager.get_suggested_advisory_ids()
        
        for advisoryId in suggestedAdvisoryIds:
                current_packages_list = packagesByAdvisoryId[advisoryId]
                print_advisory_id_details(current_packages_list, advisoryId)

        display_message_to_user(suggestedAdvisoryIds)

if __name__ == "__main__":
        main()