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
                assert isinstance(packageManager, DNFHelper)

                self.packages = {
                        "major": [],
                        "minor": [],
                        "patch": [],
                        "release": []
                }

                self.packageManager = packageManager
                self.updatesByAdvisoryId = self.packageManager.get_updates_by_partition_id()
                self.compare_updates_and_installed_packages()

                print(f"Major updates   : {len(self.packages['major'])}")
                print(f"Minor updates   : {len(self.packages['minor'])}")
                print(f"Patch updates   : {len(self.packages['patch'])}")
                print(f"Release updates : {len(self.packages['release'])}\n")

        def get_updates_by_advisory_id(self):
                assert isinstance(self.updatesByAdvisoryId, dict)

                return self.updatesByAdvisoryId
        
        def compare_updates_and_installed_packages(self):
                assert isinstance(self.packages, dict)
                assert self.packages.get("major") == []
                assert self.packages.get("minor") == []
                assert self.packages.get("patch") == []
                assert self.packages.get("release") == []

                #BUG: cleanup cache before downloading updates
                self.packageManager.download_updates()

                working_dir = self.packageManager.cache_dir
                file_list = listdir(working_dir)
                full_path_is_file = lambda file: isfile(join(working_dir, file)) and file.endswith(".rpm")

                rpm_files = [join(working_dir, file) for file in file_list if full_path_is_file(file)]

                for rpm_path in rpm_files:
                        self.evaluateRpmPackage(rpm_path)

        def evaluateRpmPackage(self, rpm_path):
                assert isinstance(rpm_path, str)
                assert rpm_path != ""
                assert rpm_path.startswith(self.packageManager.cache_dir)
                assert rpm_path.endswith(".rpm"), rpm_path

                update_info = self.packageManager.query_downloaded_package(rpm_path)

                pkg_name = update_info["Name"]
                pkg_arch = update_info["Arch"]
                pkg_signature = f"{pkg_name}.{pkg_arch}"

                installed_info = self.packageManager.query_installed_package(pkg_signature)

                if(installed_info is None):
                        return

                assert installed_info != update_info

                current_version = self.split_version_string(installed_info)
                update_version = self.split_version_string(update_info)

                major_update = current_version[0] != update_version[0]
                minor_update = current_version[1] != update_version[1]
                patch_update = current_version[2] != update_version[2]
                release_update = installed_info["Release"] != update_info["Release"]

                assert any([major_update, minor_update,patch_update, release_update])

                if (major_update):
                        self.packages["major"].append(pkg_name)
                elif (minor_update):
                        self.packages["minor"].append(pkg_name)
                elif (patch_update):
                        self.packages["patch"].append(pkg_name)
                else:
                        self.packages["release"].append(pkg_name)

        def split_version_string(self, package_info):
            version_list = f"{package_info["Version"]}.0.0".split('.')
            assert len(version_list) >= 3
            
            return version_list

        def get_suggested_advisory_ids(self):
                assert isinstance(self.updatesByAdvisoryId, dict)
                assert self.updatesByAdvisoryId != {}

                suggestedUpdates = []
                
                for advisoryId, packagesList in self.updatesByAdvisoryId.items():
                        assert isinstance(advisoryId, str)
                        assert isinstance(packagesList, list)
                        assert packagesList != []
                        assert advisoryId != ""
                        
                        if(self.evaluateUpdatePartition(packagesList)):
                                suggestedUpdates.append(advisoryId)
                
                return suggestedUpdates

        def evaluateUpdatePartition(self, packagesList):
                assert isinstance(packagesList, list)
                assert packagesList != []

                allowedAdvisoryId = False
                securityProblem = False

                for package in packagesList:
                        assert(isinstance(package, DNFUpdateEntry))
                                
                        if(package.updateUrgency > self.maxSkippableUregency):
                                securityProblem = True
                                allowedAdvisoryId = True

                        # TODO: questo fa schifo a livello di performances
                        is_patch = any([package.packageName.startswith(pkg) for pkg in self.packages['patch']])
                        is_release = any([package.packageName.startswith(pkg) for pkg in self.packages['release']])

                        if( not securityProblem and (
                                package.updateType <= self.maxAllowedUpgrade
                                or (is_patch or is_release)
                        )):
                                allowedAdvisoryId = True

                return allowedAdvisoryId