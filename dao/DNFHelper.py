import json
import os
import re
from common.costants import LIST_UPDATES_CMD
from dao.ShellInterface import ShellInterface
from dto.DNFUpdateEntry import DNFUpdateEntry


class DNFHelper:
        sh = ShellInterface()

        # TODO: rinominami per specificare si tratta delle pratizioni di aggiornamento
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
        
        def download_package(self, package_entry):
                # TODO: 
                # 1) scarica l'rpm 
                # 2) verifica che il pacchetto ci sia
                # 3) verifica che l'hash sia corretto
                # 4) se falliscono o 2) o 3) riprova (max 3 volte)
                pass

        
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
                # TODO: 
                # 1) chiama rpm con il flag --json
                # 2) verifica che l'output contenga i campi che ci interessano
                # 3) componi un oggetto custom per ritornare quei dati
                pass

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
        