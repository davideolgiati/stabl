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
                
                if (dnf_update_enrty['severity'] != "None"):
                        self.tags.append(f"severity-{dnf_update_enrty['severity'].lower()}")

def run_shell_command(command_array):
        result = subprocess.run(command_array, stdout=subprocess.PIPE)
        return result.stdout.decode('utf-8')

def process_update_list():
        output = run_shell_command(LIST_UPDATES)
        packages_list = json.loads(output)
        for package in packages_list:
                current_package = updateEntry(package)
                print(f"KEY: {current_package.key}\n\tPACKAGE_NAME: {current_package.packageName.ljust(60)}TAGS: [ {', '.join(current_package.tags)} ]")


if __name__ == "__main__":
        process_update_list()