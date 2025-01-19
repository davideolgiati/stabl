from dto.UpdateUrgency import UpdateUrgency
from dto.UpdateClassification import UpdateClassification

# (?<name>.*)[-:](?<major>\d+)\.(?<minor>\d*)\.?(?<patch>\d*)-(?<revision>.*)\.fc\d{2}\.(?<arch>.*)

updateTypeMapping = {
        'security':    UpdateClassification.SECURITY,
        'bugfix':      UpdateClassification.PATCH,
        'enhancement': UpdateClassification.MINOR,
        'unspecified': UpdateClassification.MAJOR
}

updateUrgencyMapping = {
        'critical': UpdateUrgency.CRITICAL, 
        'important': UpdateUrgency.IMPORTANT, 
        'moderate': UpdateUrgency.MODERATE, 
        'low': UpdateUrgency.LOW, 
        'none': UpdateUrgency.NONE
}

class DNFUpdateEntry:
        def __init__(self, dnf_update_enrty):
                self.key = dnf_update_enrty['name'] 
                self.packageName = dnf_update_enrty['nevra']

                updateType = dnf_update_enrty['type'].lower()
                updateUrgency = dnf_update_enrty['severity'].lower()

                self.updateType = updateTypeMapping[updateType]
                self.updateUrgency = updateUrgencyMapping[updateUrgency]
        
        def __eq__(self, other): 
                if self.__class__ is not other.__class__:
                        raise TypeError
                
                return (
                        self.key == other.key 
                        and self.packageName == other.packageName
                        and self.updateType == other.updateType
                        and self.updateUrgency == other.updateUrgency
                )
                