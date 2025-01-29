from dto.UpdateUrgency import UpdateUrgency
from dto.UpdateClassification import UpdateClassification

updateTypeMapping = {
        'security':    UpdateClassification.SECURITY,
        'bugfix':      UpdateClassification.PATCH,
        'enhancement': UpdateClassification.MINOR,
        'unspecified': UpdateClassification.MAJOR
}

updateUrgencyMapping = {
        'critical':    UpdateUrgency.CRITICAL, 
        'important':   UpdateUrgency.IMPORTANT, 
        'moderate':    UpdateUrgency.MODERATE, 
        'low':         UpdateUrgency.LOW, 
        'none':        UpdateUrgency.NONE
}

class DNFUpdateEntry:
        def __init__(self, dnf_update_enrty):
                assert isinstance(dnf_update_enrty, dict)
                
                required_keys = ['name', 'nevra', 'type', 'severity']
                object_keys = dnf_update_enrty.keys()

                assert all(isinstance(key, str) for key in object_keys)
                assert all(key in object_keys for key in required_keys)
                assert all(isinstance(dnf_update_enrty[key],str) for key in object_keys)

                self.key = dnf_update_enrty['name'] 
                assert self.key != ""

                self.packageName = dnf_update_enrty['nevra']
                assert self.packageName != ""

                updateType = dnf_update_enrty['type'].lower()
                self.updateType = updateTypeMapping[updateType]
                assert isinstance(self.updateType, UpdateClassification)
                
                updateUrgency = dnf_update_enrty['severity'].lower()
                self.updateUrgency = updateUrgencyMapping[updateUrgency]
                assert isinstance(self.updateUrgency, UpdateUrgency)
                        

        def __eq__(self, other): 
                if self.__class__ is not other.__class__:
                        raise TypeError
                
                return all([
                        self.key == other.key,
                        self.packageName == other.packageName,
                        self.updateType == other.updateType,
                        self.updateUrgency == other.updateUrgency
                ])