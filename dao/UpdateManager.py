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
                        "major": 0,
                        "minor": 0,
                        "patch": 0,
                        "release": 0
                }

                self.lock = threading.Lock()
                self.packageManager = packageManager
                self.updatesByAdvisoryId = self.packageManager.get_updates_by_partition_id()



        def get_updates_by_advisory_id(self):
                assert isinstance(self.updatesByAdvisoryId, dict)

                return self.updatesByAdvisoryId
         

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
                
                print("\n")
                print(f"Major updates   : {self.packages['major']}")
                print(f"Minor updates   : {self.packages['minor']}")
                print(f"Patch updates   : {self.packages['patch']}")
                print(f"Release updates : {self.packages['release']}\n")

                return suggestedUpdates

        def evaluateUpdatePartition(self, packagesList):
                assert isinstance(packagesList, list)
                assert packagesList != []

                allowedAdvisoryId = True
                securityUpdate = False

                for package in packagesList:
                        assert(isinstance(package, DNFUpdateEntry))
                                
                        if(package.updateUrgency > self.maxSkippableUregency):
                                securityUpdate = True

                        if(package.updateType > self.maxAllowedUpgrade):
                                allowedAdvisoryId = False or securityUpdate

                        if (package.updateType == UpdateClassification.MAJOR):
                                self.packages['major'] += 1
                        elif (package.updateType ==  UpdateClassification.MINOR):
                                self.packages['minor'] += 1
                        elif (package.updateType == UpdateClassification.PATCH):
                                self.packages['patch'] += 1
                        else:
                                self.packages['release'] += 1

                return allowedAdvisoryId