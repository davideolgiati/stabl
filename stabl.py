import subprocess
import json

LIST_UPDATES = ["dnf", "updateinfo", "list", "--json"]

class updateEntry:
        def __init__(self, dnf_update_enrty):
                self.tags = []
                self.key = dnf_update_enrty['name'] 
                self.packageName = dnf_update_enrty['nevra']

                if (dnf_update_enrty['type'] != "unspecified"):
                        self.tags.append(dnf_update_enrty['type'])
                else:
                        self.tags.append('major')
                
                if (dnf_update_enrty['severity'] == "None"):
                        self.tags.append("no-priority")
                else:
                        self.tags.append(f"{dnf_update_enrty['severity'].lower()}-priority")

def run_shell_command(command_array):
        result = subprocess.run(command_array, stdout=subprocess.PIPE)
        return result.stdout.decode('utf-8')

def process_update_list():
        output = run_shell_command(LIST_UPDATES)
        packages_list = json.loads(output)

        updateGruops = {}
        suggestedUpdates = []

        for package in packages_list:
                current_package = updateEntry(package)
                if (current_package.key not in updateGruops):
                        updateGruops[current_package.key] = [current_package]
                else:
                        updateGruops[current_package.key].append(current_package)

        for key, packages in updateGruops.items():
                print(f"{key}:")
                addKey = False
                securityProblem = False
                for package in packages:
                        print(f"\t{package.packageName.ljust(60)} [ {', '.join(package.tags)} ]")
                        
                        if('security' in package.tags or ('no-priority' not in package.tags)):
                                securityProblem = True
                        
                        if('bugfix' in package.tags or securityProblem):
                                addKey = True
                        else:
                                addKey = False
                if(addKey):
                        suggestedUpdates.append(key)
        
        if(suggestedUpdates != []):
                print(f"suggested updates: sudo dnf update --advisory={','.join(suggestedUpdates)}")
        else:
                print(f"no suggested updates found") 


if __name__ == "__main__":
        process_update_list()