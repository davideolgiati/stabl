import json
from common.costants import LIST_UPDATES_CMD
from dao.ShellInterface import ShellInterface
from dto.UpdateEntry import UpdateEntry


class DNFHelper:
        sh = ShellInterface()

        def get_updates(self):
                output = self.sh.run(LIST_UPDATES_CMD)
                packages_list = json.loads(output)

                updateGruops = {}

                for package in packages_list:
                        current_package = UpdateEntry(package)
                        if (current_package.key not in updateGruops):
                                updateGruops[current_package.key] = [current_package]
                        else:
                                updateGruops[current_package.key].append(current_package)
                
                return updateGruops