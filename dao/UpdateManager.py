from os import listdir
from os.path import isfile, join

from dto.UpdateUrgency import UpdateUrgency
from dto.UpdateClassification import UpdateClassification


class UpdateManager():
        maxAllowedUpgrade = UpdateClassification.PATCH
        maxSkippableUregency = UpdateUrgency.NONE

        def __init__(self, packageManager):
                self.packageManager = packageManager
                self.updatesByAdvisoryId = self.packageManager.get_updates() # Questo cambiera' nome
                self.compare_updates_and_installed_packages()

        def get_updates_by_advisory_id(self):
                return self.updatesByAdvisoryId
        
        def compare_updates_and_installed_packages(self):
                self.packages = {}
                self.packageManager.download_updates()

                rpm_files = [join("/tmp/stabl/", f) for f in listdir("/tmp/stabl/") if isfile(join("/tmp/stabl/", f))]

                for rpm_path in rpm_files:
                        try: 
                                update_info = self.packageManager.query_downloaded_package(rpm_path)

                                pkg_name = update_info["Name"]

                                installed_info = self.packageManager.query_installed_package(pkg_name)

                                installed_version_raw = f"{installed_info["Version"]}.0.0".split('.')
                                update_version_raw = f"{update_info["Version"]}.0.0".split('.')

                                major_update = installed_version_raw[0] != update_version_raw[0]
                                minor_update = installed_version_raw[1] != update_version_raw[1]

                                self.packages[pkg_name] = {
                                        "installed" : {
                                                "version": installed_info["Version"],
                                                "release": installed_info["Release"]
                                        },
                                        "update" : {
                                                "version": update_info["Version"],
                                                "release": update_info["Release"]
                                        },
                                        "updateNeeded": not (major_update or minor_update)
                                } 
                        except:
                                pass

                print([name for name, props in self.packages.items() if props["updateNeeded"]])
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