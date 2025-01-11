from dto.UpdateClassification import UpdateClassification


updateTypeMapping = {
        'security':    UpdateClassification.SECURITY,
        'bugfix':      UpdateClassification.PATCH,
        'enhancement': UpdateClassification.MINOR,
        'unspecified': UpdateClassification.MAJOR
}

class DNFUpdateEntry:
        def __init__(self, dnf_update_enrty):
                self.tags = []
                self.key = dnf_update_enrty['name'] 
                self.packageName = dnf_update_enrty['nevra']

                updateType = dnf_update_enrty['type']
                self.updateType = updateTypeMapping[updateType]
                
                if (dnf_update_enrty['severity'] == "None"):
                        self.tags.append("no-priority")
                else:
                        self.tags.append(f"{dnf_update_enrty['severity'].lower()}-priority")