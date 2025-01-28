from os import listdir
from os.path import isfile, join

from dto.DNFUpdateEntry import DNFUpdateEntry
from dto.UpdateUrgency import UpdateUrgency
from dto.UpdateClassification import UpdateClassification


class UpdateManager():
        maxAllowedUpgrade = UpdateClassification.PATCH
        maxSkippableUregency = UpdateUrgency.NONE

        def __init__(self, packageManager):
                assert(packageManager is not None)

                self.packageManager = packageManager
                self.updatesByAdvisoryId = self.packageManager.get_updates() # Questo cambiera' nome
                self.compare_updates_and_installed_packages()

        def get_updates_by_advisory_id(self):
                assert(self.updatesByAdvisoryId is not None)
                assert(isinstance(self.updatesByAdvisoryId, dict))

                return self.updatesByAdvisoryId
        
        def compare_updates_and_installed_packages(self):
                self.packages = {
                        "major": [],
                        "minor": [],
                        "patch": [],
                        "release": []
                }
                
                self.packageManager.download_updates()

                rpm_files = [join("/tmp/stabl/", f) for f in listdir("/tmp/stabl/") if isfile(join("/tmp/stabl/", f))]

                for rpm_path in rpm_files:
                        assert(rpm_path is not None)
                        assert(isinstance(rpm_path, str))
                        assert(rpm_path != "")

                        try: 
                                update_info = self.packageManager.query_downloaded_package(rpm_path)

                                assert(update_info is not None)
                                assert(isinstance(update_info, dict))
                                assert("Name" in update_info)
                                assert("Version" in update_info)
                                assert("Release" in update_info)
                                assert(isinstance(update_info["Name"], str))
                                assert(isinstance(update_info["Version"], str))
                                assert(isinstance(update_info["Release"], str))

                                pkg_name = update_info["Name"]
                                installed_info = self.packageManager.query_installed_package(pkg_name)

                                assert(installed_info is not None)
                                assert(isinstance(installed_info, dict))
                                assert("Name" in installed_info)
                                assert("Version" in installed_info)
                                assert("Release" in installed_info)
                                assert(isinstance(installed_info["Name"], str))
                                assert(isinstance(installed_info["Version"], str))
                                assert(isinstance(installed_info["Release"], str))

                                assert(installed_info != update_info)

                                installed_version_raw = f"{installed_info["Version"]}.0.0".split('.')
                                update_version_raw = f"{update_info["Version"]}.0.0".split('.')

                                assert(len(installed_version_raw) > 3)
                                assert(len(update_version_raw) > 3)

                                major_update = installed_version_raw[0] != update_version_raw[0]
                                minor_update = installed_version_raw[1] != update_version_raw[1]
                                patch_update = installed_version_raw[2] != update_version_raw[2]
                                release_update = installed_info["Release"] != update_info["Release"]

                                assert(any([
                                        major_update, minor_update,
                                        patch_update, release_update
                                ]))

                                if (major_update):
                                        self.packages["major"].append(pkg_name)
                                elif (minor_update):
                                        self.packages["minor"].append(pkg_name)
                                elif (patch_update):
                                        self.packages["patch"].append(pkg_name)
                                else:
                                        self.packages["release"].append(pkg_name)
                        except:
                                pass

                print(self.packages.items())
                pass

        def get_suggested_advisory_ids(self):
                assert(self.updatesByAdvisoryId is not None)

                suggestedUpdates = []
                
                for advisoryId, packagesList in self.updatesByAdvisoryId.items():
                        assert(advisoryId is not None)
                        assert(isinstance(advisoryId, str))
                        assert(packagesList is not None)
                        assert(isinstance(packagesList, list))
                        assert(packagesList != [])

                        if(self.evaluateUpdatePartition(packagesList)):
                                suggestedUpdates.append(advisoryId)
                
                return suggestedUpdates

        def evaluateUpdatePartition(self, packagesList):
            assert(packagesList is not None)
            assert(isinstance(packagesList, list))
            
            allowedAdvisoryId = False
            securityProblem = False

            for package in packagesList:
                    assert(isinstance(package, DNFUpdateEntry))
                                
                    if(package.updateUrgency > self.maxSkippableUregency):
                            securityProblem = True
                            allowedAdvisoryId = True

                    if( not securityProblem
                        and package.updateType <= self.maxAllowedUpgrade):
                            allowedAdvisoryId = True

            return allowedAdvisoryId