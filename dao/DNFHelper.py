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
                self.local_rpm_cache = [file for file in os.listdir("/tmp/stabl/") if is_valid_rpm_file_path(file)]

        # TODO: rinominami per specificare si tratta delle partizioni di aggiornamento
        def get_updates(self):
                assert(LIST_UPDATES_CMD is not None)
                assert(isinstance(LIST_UPDATES_CMD, list))

                raw_json_output = self.sh.run(LIST_UPDATES_CMD)
                
                assert(raw_json_output is not None)
                assert(isinstance(raw_json_output, str))
                assert(raw_json_output != "")

                packages_list = json.loads(raw_json_output)

                assert(isinstance(packages_list, list))

                updateGruops = {}

                for package in packages_list:
                        assert(package is not None)
                        assert(isinstance(package, dict))
                        
                        current_package = DNFUpdateEntry(package)
                        if (current_package.key not in updateGruops):
                                updateGruops[current_package.key] = [current_package]
                        else:
                                updateGruops[current_package.key].append(current_package)
                
                return updateGruops
        
        def download_updates(self):
                assert(DOWNLOAD_UPGRADE is not None)
                assert(isinstance(DOWNLOAD_UPGRADE, list))

                self.sh.run(DOWNLOAD_UPGRADE)
        
        
        def query_downloaded_package(self, package_path):
                assert(package_path is not None)
                assert(isinstance(package_path, str))
                assert(is_valid_rpm_file_path(package_path))

                # TODO: specific errors
                if(not os.path.isfile(package_path)):
                        raise ValueError(f"{package_path} doesn't exist")                    

                if(not is_file_rpm(package_path)):
                        raise ValueError(f"RPM validation failed on {package_path}") 

                return self.query_package_info(package_path)


        def query_installed_package(self, package_name: str):
                assert(package_name is not None)
                assert(package_name != "")
                assert(isinstance(package_name, str))

                return self.query_package_info(package_name)


        # TODO: https://docs.python.org/3/library/multiprocessing.html#exchanging-objects-between-processes
        def query_package_info(self, package_entry):
                assert(package_entry is not None)
                assert(isinstance(package_entry, str))
                assert(package_entry != "")

                pkg_name_regex = r'^[A-Za-z0-1]+(\-[A-Za-z0-1]+)*$'
                pkg_version_regex = r'^\d+(\.\d+){0,2}$'
                
                raw_rpm_output = self.sh.run(INSPECT_PKG(package_entry))
                rpm_pkg_property_dict = json.loads(raw_rpm_output)

                required_properties = [ "Name", "Version", "Release" ]
                output_dictionary = {} # TODO: questo va reso una classe

                for key in required_properties:
                       current_value = rpm_pkg_property_dict.get(key)
                       assert(current_value is not None)
                       assert(isinstance(current_value, str))
                       assert(current_value != "")
                       
                       output_dictionary[key] = current_value

                assert(re.search(pkg_name_regex, output_dictionary["Name"]))
                assert(re.search(pkg_version_regex, output_dictionary["Version"]))                

                return output_dictionary
        

def is_valid_rpm_file_path(path):
        assert(path is not None)
        assert(isinstance(path, str))

        if re.search(r'\.rpm$', path, re.IGNORECASE):
                return True
        else:
                return False

def is_file_rpm(path):
        assert(path is not None)
        assert(isinstance(path, str))
        assert(path != "")

        rpm_magic_bytes = b'\xed\xab\xee\xdb'
        with open(path, 'rb') as fp:
                file_magic_bytes = fp.read(4)

        return file_magic_bytes == rpm_magic_bytes
        