import json
from dto.ManagedDNFException import ManagedException
from dto.UpdateUrgency import UpdateUrgency
from dto.UpdateClassification import UpdateClassification

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
                DNFUpdateEntry.validate_record(dnf_update_enrty)

                self.key = dnf_update_enrty['name'] 
                self.packageName = dnf_update_enrty['nevra']

                updateType = dnf_update_enrty['type'].lower()
                updateUrgency = dnf_update_enrty['severity'].lower()

                self.updateType = updateTypeMapping[updateType]
                self.updateUrgency = updateUrgencyMapping[updateUrgency]

                assert(self.key is not None)
                assert(isinstance(self.key, str))
                assert(self.key != "")

                assert(self.packageName is not None)
                assert(isinstance(self.packageName, str))
                assert(self.packageName != "")

                assert(self.updateType is not None)
                assert(isinstance(self.updateType, UpdateClassification))
                
                assert(self.updateUrgency is not None)
                assert(isinstance(self.updateUrgency, UpdateUrgency))

        
        @staticmethod
        def validate_record(dnf_update_enrty):
                if not isinstance(dnf_update_enrty, dict):
                        raise ManagedException(
                                ["Provided value is not an object"],
                                dnf_update_enrty
                        )
                
                required_keys = ['name', 'nevra', 'type', 'severity']
                object_keys = set(dnf_update_enrty.keys())

                evaluation_fn = lambda key: validate_key(object_keys, key) 

                errors = [evaluation_fn(k) for k in required_keys]
                errors = [e for e in errors if e is not None]

                if errors != []:
                        raise ManagedException(
                                errors,
                                dnf_update_enrty
                        )
                        

        def __eq__(self, other): 
                if self.__class__ is not other.__class__:
                        raise TypeError
                
                return (
                        self.key == other.key 
                        and self.packageName == other.packageName
                        and self.updateType == other.updateType
                        and self.updateUrgency == other.updateUrgency
                )
                

def validate_key(test_keys, key):
        assert(key is not None)
        assert(isinstance(key, str))
        assert(key != "")

        assert(test_keys is not None)
        assert(isinstance(test_keys, set))

        if key not in test_keys:
                return f"key '{key}' is missing in provided record"