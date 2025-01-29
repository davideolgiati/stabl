from dao.UpdateManager import UpdateManager
from dao.DNFHelper import DNFHelper

def process_update_list():
        packageManager = DNFHelper()
        updateManager = UpdateManager(packageManager)

        packagesByAdvisoryId = updateManager.get_updates_by_advisory_id()
        suggestedAdvisoryIds = updateManager.get_suggested_advisory_ids()
        
        for advisoryId in suggestedAdvisoryIds:
                printBuffer = ""
                for package in packagesByAdvisoryId[advisoryId]:
                        printBuffer += f"\t{package.packageName.ljust(60)}\n"
                
                print(f"Advisory Id: \"{advisoryId}\" () \n{printBuffer}")


        if(suggestedAdvisoryIds != []):
                print(f"suggested updates: sudo dnf update --advisory={','.join(suggestedAdvisoryIds)}")
        else:
                print(f"no suggested updates found") 


if __name__ == "__main__":
        process_update_list()