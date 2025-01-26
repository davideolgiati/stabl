import json
import os
import re

from dao.ShellInterface import ShellInterface
from dto.DNFUpdateEntry import DNFUpdateEntry

from common.costants import LIST_UPDATES_CMD, DOWNLOAD_UPGRADE, INSPECT_PKG


class DNFHelper:
        def __init__(self):
                self.sh = ShellInterface()
                if(not os.path.isdir("/tmp/stabl/")): #TODO: da mockare nei test
                       os.mkdir("/tmp/stabl/") #TODO: da mockare nei test

                # Buon usecase per la tie
                local_rpm_cache = [file for file in os.listdir("/tmp/stabl/") if is_valid_rpm_file_path(file)]

        # TODO: rinominami per specificare si tratta delle partizioni di aggiornamento
        def get_updates(self):
                output = self.sh.run(LIST_UPDATES_CMD)
                packages_list = json.loads(output)

                updateGruops = {}

                for package in packages_list:
                        current_package = DNFUpdateEntry(package)
                        if (current_package.key not in updateGruops):
                                updateGruops[current_package.key] = [current_package]
                        else:
                                updateGruops[current_package.key].append(current_package)
                
                return updateGruops
        
        def download_package(self, package_name):
                if(f"{package_name}.rpm" in self.local_rpm_cache):
                        return
                
                self.sh.run(DOWNLOAD_UPGRADE(package_name)) #TODO: da mockare nei test
                
                rpm_pkg_path = f"/tmp/stabl/{package_name}.rpm"

                if(os.path.isfile(rpm_pkg_path)): #TODO: da mockare nei test
                        # TODO: specific error
                        raise FileNotFoundError
                
                return rpm_pkg_path

        
        def query_downloaded_package(self, package_path):
                if (package_path is None):
                        # TODO: specific error
                        raise ValueError
                
                sanitized_pkg_path = package_path.strip()

                if(not is_valid_rpm_file_path(sanitized_pkg_path)):
                        # TODO: specific error
                        raise ValueError

                if(not os.path.isfile(sanitized_pkg_path)):
                        # TODO: specific error
                        raise ValueError                    

                if(not is_file_rpm(sanitized_pkg_path)):
                        # TODO: specific error
                        raise ValueError 

                return self.query_package_info(sanitized_pkg_path)

        def query_installed_package(self, package_name):
                if (package_name is None):
                        # TODO: specific error
                        raise ValueError

                sanitized_pkg_name = package_name.strip()

                if(sanitized_pkg_name == ""):
                        # TODO: specific error
                        raise ValueError

                return self.query_package_info(sanitized_pkg_name)

        def query_package_info(self, package_entry):
                raw_rpm_output = self.sh.run(INSPECT_PKG(package_entry))
                rpm_pkg_property_dict = json.loads(raw_rpm_output)

                required_properties = [
                        "Name", "Version", "Release", "Buildtime","Arch"
                ]

                output_dictionary = {} # TODO: questo va reso una classe

                for key in required_properties:
                       current_value = rpm_pkg_property_dict.get(key)
                       if current_value is None:
                              # TODO: specific error
                              raise ValueError
                       
                       output_dictionary[key] = current_value

                return output_dictionary
        

def is_valid_rpm_file_path(path):
    if re.search(r'\.rpm$', path, re.IGNORECASE):
        return True
    else:
        return False

def is_file_rpm(path):
        rpm_magic_bytes = b'\xed\xab\xee\xdb'
        with open(path, 'rb') as fp:
                file_magic_bytes = fp.read(4)

        return file_magic_bytes == rpm_magic_bytes
        