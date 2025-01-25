import json
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

        def query_package_info(self, package_entry):
                # TODO: 
                # 1) chiama rpm con il flag --json
                # 2) verifica che l'output contenga i campi che ci interessano
                # 3) componi un oggetto custom per ritornare quei dati
                pass