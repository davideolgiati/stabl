from dto.UpdateClassification import UpdateClassification


class UpdateManager():
        maxAllowedUpgrade = UpdateClassification.PATCH

        def __init__(self, packageManager):
                self.packageManager = packageManager
                self.updatesByAdvisoryId = self.packageManager.get_updates()
        
        def get_valid_update_list(self):
                suggestedUpdates = []
                
                for advisoryId, packagesList in self.updatesByAdvisoryId.items():
                        allowedAdvisoryId = False
                        securityProblem = False

                        for package in packagesList:
                                # TODO: logica per valutare la priority
                                if(not securityProblem):
                                        if (package.updateType <= self.maxAllowedUpgrade):
                                                allowedAdvisoryId = True
                                        else:
                                                allowedAdvisoryId = False
                                        
                                        if (package.updateType == UpdateClassification.SECURITY):
                                                securityProblem = True
                        
                        if(allowedAdvisoryId):
                                suggestedUpdates.append(advisoryId)
                
                return suggestedUpdates