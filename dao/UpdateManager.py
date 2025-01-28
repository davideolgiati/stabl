from os import listdir
from os.path import isfile, join, isdir

from dao.DNFHelper import DNFHelper
from dto.DNFUpdateEntry import DNFUpdateEntry
from dto.UpdateUrgency import UpdateUrgency
from dto.UpdateClassification import UpdateClassification


class UpdateManager():
        maxAllowedUpgrade = UpdateClassification.PATCH
        maxSkippableUregency = UpdateUrgency.NONE

        def __init__(self, packageManager):
                assert packageManager is not None, "packageManager must be valorized"
                assert isinstance(packageManager, DNFHelper), "packageManager must be instance of DNFHelper"

                self.packageManager = packageManager
                self.updatesByAdvisoryId = self.packageManager.get_updates() # Questo cambiera' nome
                self.compare_updates_and_installed_packages()

        def get_updates_by_advisory_id(self):
                assert self.updatesByAdvisoryId is not None, "updatesByAdvisoryId must be valorized"
                assert isinstance(self.updatesByAdvisoryId, dict), "updatesByAdvisoryId must be a dictionary"

                return self.updatesByAdvisoryId
        
        def compare_updates_and_installed_packages(self):
                self.packages = {
                        "major": [],
                        "minor": [],
                        "patch": [],
                        "release": []
                }

                self.packageManager.download_updates()

                assert isdir("/tmp/stabl/"), "\"/tmp/stabl\" must exist on the system"
                rpm_files = [join("/tmp/stabl/", f) for f in listdir("/tmp/stabl/") if isfile(join("/tmp/stabl/", f))]

                for rpm_path in rpm_files:
                        self.evaluateRpmPackage(rpm_path)

                print(self.packages.items())
                pass

        def evaluateRpmPackage(self, rpm_path):
                assert rpm_path is not None, "the rpm packet file path must be valorized"
                assert isinstance(rpm_path, str), "the rpm packet file path must be a string"
                assert rpm_path != "", "the rpm packet file path must contain a value"
                assert rpm_path.startswith("/tmp/stabl/"), "the rpm packet file path must start with the agreed prefix"

                update_info = self.packageManager.query_downloaded_package(rpm_path)

                pkg_name = update_info["Name"]
                pkg_arch = update_info["Arch"]
                installed_info = self.packageManager.query_installed_package(f"{pkg_name}.{pkg_arch}")

                if(installed_info is None):
                        return

                assert installed_info != update_info, "installed package info and update info must be different"

                current_version_raw = f"{installed_info["Version"]}.0.0".split('.')
                update_version_raw = f"{update_info["Version"]}.0.0".split('.')
                current_version_check = len(current_version_raw) >= 3
                update_version_check = len(update_version_raw) >= 3

                assert current_version_check, "Installed package version must be composed by 3 digits"
                assert update_version_check, "Update package version must be composed by 3 digits"

                major_update = current_version_raw[0] != update_version_raw[0]
                minor_update = current_version_raw[1] != update_version_raw[1]
                patch_update = current_version_raw[2] != update_version_raw[2]
                release_update = installed_info["Release"] != update_info["Release"]

                version_check = any([major_update, minor_update,patch_update, release_update])
                assert version_check, "Update version must be in at least one category"

                if (major_update):
                        self.packages["major"].append(pkg_name)
                elif (minor_update):
                        self.packages["minor"].append(pkg_name)
                elif (patch_update):
                        self.packages["patch"].append(pkg_name)
                else:
                        self.packages["release"].append(pkg_name)

        def get_suggested_advisory_ids(self):
                assert self.updatesByAdvisoryId is not None, "updatesByAdvisoryId must be valorized"

                suggestedUpdates = []
                
                for advisoryId, packagesList in self.updatesByAdvisoryId.items():
                        assert advisoryId is not None, "advisoryId must be valorized"
                        assert isinstance(advisoryId, str), "advisoryId must be a string"
                        
                        if(self.evaluateUpdatePartition(packagesList)):
                                suggestedUpdates.append(advisoryId)
                
                return suggestedUpdates

        def evaluateUpdatePartition(self, packagesList):
                assert packagesList is not None, "packagesList must be valorized"
                assert isinstance(packagesList, list), "packagesList must be a list"
                assert packagesList != [], "packagesList must not be empty"

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