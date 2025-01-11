from common.costants import BUGFIX_UPDATE_TAG, SECURITY_UPDATE_TAG
from dto.UpdateClassification import UpdateClassification


class UpdateManager():
        maxAllowedUpgrade = UpdateClassification.PATCH

        def __init__(self, packageManager):
                self.packageManager = packageManager
        
        def get_valid_update_list(self):
                suggestedUpdates = []
                updateGruops = self.packageManager.get_updates()

                for key, packages in updateGruops.items():
                        addKey = False
                        securityProblem = False

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
                        
                        if(addKey):
                                suggestedUpdates.append(key)
                
                return suggestedUpdates