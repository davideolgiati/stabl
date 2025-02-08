from common.rpm.properties import unpack_version_string
from dto.enums.UpdateUrgency import UpdateUrgency
from dto.enums.UpdateClassification import UpdateClassification

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
        
        def set_new_version(self, rpm):
                self.new_version = rpm["Version"]
                self.new_release = rpm["Release"]

        def set_current_version(self, rpm):
                self.current_version = rpm["Version"]
                self.current_release = rpm["Release"]

        def compute_update_type(self):
                update_info = unpack_version_string(self.current_version)
                installed_info = unpack_version_string(self.new_version)

                assert installed_info != update_info or self.current_release != self.new_release

                major_update   = installed_info["Major"] != update_info["Major"]
                minor_update   = installed_info["Minor"] != update_info["Minor"]
                patch_update   = installed_info["Patch"] != update_info["Patch"]
                release_update = self.current_release != self.new_release

                assert any([major_update, minor_update, patch_update, release_update])

                if (major_update):
                        new_update_type = UpdateClassification.MAJOR
                elif (minor_update):
                        new_update_type = UpdateClassification.MINOR
                elif (patch_update):
                        new_update_type = UpdateClassification.PATCH
                else:
                        new_update_type = UpdateClassification.RELEASE
                
                self.updateType = new_update_type

        def __eq__(self, other): 
                if self.__class__ is not other.__class__:
                        raise TypeError
                
                return all([
                        self.key == other.key,
                        self.packageName == other.packageName,
                        self.updateType == other.updateType,
                        self.updateUrgency == other.updateUrgency
                ])