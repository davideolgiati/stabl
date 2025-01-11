from common.costants import BUGFIX_UPDATE_TAG, SECURITY_UPDATE_TAG
from dao.DNFHelper import DNFHelper

def process_update_list(noFilter):
        packageManager = DNFHelper()
        suggestedUpdates = []
        updateGruops = packageManager.get_updates()

        for key, packages in updateGruops.items():
                addKey = False
                securityProblem = False
                printBuffer = ""

                for package in packages:
                        if(not securityProblem):
                                if(SECURITY_UPDATE_TAG in package.tags):
                                        securityProblem = True
                                        addKey = True
                        
                                if('no-priority' not in package.tags):
                                        securityProblem = True
                                        addKey = True

                                if(BUGFIX_UPDATE_TAG in package.tags or securityProblem):
                                        addKey = True
                                else:
                                        addKey = False
                                
                        if(addKey or noFilter):        
                                printBuffer += f"\t{package.packageName.ljust(60)} [ {', '.join(package.tags)} ]\n"
                
                if(addKey):
                        suggestedUpdates.append(key)
                
                if(printBuffer != ""):
                        print(f"{key}\n{printBuffer}")
        
        if(suggestedUpdates != []):
                print(f"suggested updates: sudo dnf update --advisory={','.join(suggestedUpdates)}")
        else:
                print(f"no suggested updates found") 


if __name__ == "__main__":
        process_update_list(False)