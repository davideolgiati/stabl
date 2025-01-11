from common.costants import BUGFIX_UPDATE_TAG, SECURITY_UPDATE_TAG
from dao.UpdateManager import UpdateManager
from dao.DNFHelper import DNFHelper

def process_update_list(noFilter):
        packageManager = DNFHelper()
        updateManager = UpdateManager(packageManager)
        suggestedUpdates = updateManager.get_valid_update_list()
        
        
        # if(addKey or noFilter):        
        #         printBuffer += f"\t{package.packageName.ljust(60)} [ {', '.join(package.tags)} ]\n"


        if(suggestedUpdates != []):
                print(f"suggested updates: sudo dnf update --advisory={','.join(suggestedUpdates)}")
        else:
                print(f"no suggested updates found") 


if __name__ == "__main__":
        process_update_list(False)