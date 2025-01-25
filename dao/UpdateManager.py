from dto.UpdateUrgency import UpdateUrgency
from dto.UpdateClassification import UpdateClassification


class UpdateManager():
        maxAllowedUpgrade = UpdateClassification.PATCH
        maxSkippableUregency = UpdateUrgency.NONE

        def __init__(self, packageManager):
                self.packageManager = packageManager
                self.updatesByAdvisoryId = self.packageManager.get_updates() # Questo cambiera' nome

        def get_updates_by_advisory_id(self):
                return self.updatesByAdvisoryId
        
        def compare_updates_and_installed_packages(self):
                # TODO:
                # 1) for-each pacchetto nella lista scarico l'rpm
                # 2) estraggo le informazioni dall'rpm 
                # 3) estraggo le informazioni dalla versione installata
                # 4) determino che genere di aggiornamento e' conforntando le due versioni
                pass

        def get_suggested_advisory_ids(self):
                suggestedUpdates = []
                
                for advisoryId, packagesList in self.updatesByAdvisoryId.items():
                        allowedAdvisoryId = False
                        securityProblem = False

                        for package in packagesList:
                                if(package.updateUrgency > self.maxSkippableUregency):
                                        securityProblem = True
                                        allowedAdvisoryId = True

                                if(not securityProblem
                                   and package.updateType <= self.maxAllowedUpgrade):
                                        allowedAdvisoryId = True
                                        
                        
                        if(allowedAdvisoryId):
                                suggestedUpdates.append(advisoryId)
                
                return suggestedUpdates