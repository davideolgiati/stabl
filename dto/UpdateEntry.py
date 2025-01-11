class UpdateEntry:
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