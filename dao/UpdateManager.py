from os import listdir
import os
from os.path import isfile, join
import threading
import time

from common.logger import log_timed_execution
from dao.DNF import DNF
from dto.DNFUpdateEntry import DNFUpdateEntry
from dto.UpdateUrgency import UpdateUrgency
from dto.UpdateClassification import UpdateClassification


class UpdateManager():
        maxAllowedUpgrade = UpdateClassification.PATCH
        maxSkippableUregency = UpdateUrgency.NONE

        def __init__(self, packageManager):
                assert isinstance(packageManager, DNF)

                self.packages = {
                        "major": [],
                        "minor": [],
                        "patch": [],
                        "release": []
                }

                self.lock = threading.Lock()
                self.packageManager = packageManager
                self.updatesByAdvisoryId = self.packageManager.get_updates_by_partition_id()
                self.compare_updates_and_installed_packages()

                print("\n")
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
                

                self.cleanup_environment()
                self.packageManager.download_updates()

                working_dir = self.packageManager.cache_dir
                file_list = listdir(working_dir)
                full_path_is_file = lambda file: isfile(join(working_dir, file)) and file.endswith(".rpm")

                rpm_files = [join(working_dir, file) for file in file_list if full_path_is_file(file)]

                thread_list = []
                for rpm_path in rpm_files:
                        thread = threading.Thread(target=self.evaluateRpmPackage, args=(rpm_path,))
                        thread_list.append(thread)
                        thread.start()
                
                for thread in thread_list:
                        thread.join()

        @log_timed_execution("Cleaning environment")
        def cleanup_environment(self):
            thread_list = []

            for file in os.listdir(self.packageManager.cache_dir):
                    thread = threading.Thread(target=self.evaluate_file_for_deletion, args=(file,))
                    thread_list.append(thread)
                    thread.start()
                
            for thread in thread_list:
                    thread.join()

        def evaluate_file_for_deletion(self, file):
                assert isinstance(file, str)
                assert file != ""
                assert isinstance(self.packageManager.cache_dir, str)
                assert self.packageManager.cache_dir != ""

                if not file.endswith(".rpm"):
                        return
                
                full_path = os.path.join(self.packageManager.cache_dir, file)
                rpm_properties =self.packageManager.query_downloaded_package(full_path)
                
                assert isinstance(rpm_properties, dict)
                assert rpm_properties != {}
                assert "Name" in rpm_properties
                assert "Arch" in rpm_properties
                assert "Version" in rpm_properties
                assert "Release" in rpm_properties

                pkg_signature = f"{rpm_properties["Name"]}.{rpm_properties["Arch"]}"
                installed_info = self.packageManager.query_installed_package(pkg_signature)

                assert isinstance(installed_info, dict)
                assert installed_info != {}
                assert "Version" in installed_info
                assert "Release" in installed_info

                same_version = installed_info["Version"] == rpm_properties["Version"]
                same_release = installed_info["Release"] == rpm_properties["Release"]

                if not (same_version and same_release):
                        return
                        
                os.remove(full_path)

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

                major_update   = installed_info["Major"] != update_info["Major"]
                minor_update   = installed_info["Minor"] != update_info["Minor"]
                patch_update   = installed_info["Patch"] != update_info["Patch"]
                release_update = installed_info["Release"] != update_info["Release"]

                assert any([major_update, minor_update, patch_update, release_update])

                with self.lock:
                        if (major_update):
                                self.packages["major"].append(pkg_name)
                        elif (minor_update):
                                self.packages["minor"].append(pkg_name)
                        elif (patch_update):
                                self.packages["patch"].append(pkg_name)
                        else:
                                self.packages["release"].append(pkg_name)

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

                for package in packagesList:
                        assert(isinstance(package, DNFUpdateEntry))
                                
                        if(package.updateUrgency > self.maxSkippableUregency):
                                allowedAdvisoryId = True

                        # TODO: questo fa schifo a livello di performances
                        is_patch = any([package.packageName.startswith(pkg) for pkg in self.packages['patch']])
                        is_release = any([package.packageName.startswith(pkg) for pkg in self.packages['release']])

                        if(package.updateType <= self.maxAllowedUpgrade
                           or (is_patch or is_release)):
                                allowedAdvisoryId = True

                return allowedAdvisoryId