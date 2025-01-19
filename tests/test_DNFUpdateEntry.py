# classes to be tested 
from dto.DNFUpdateEntry import DNFUpdateEntry
from dto.UpdateClassification import UpdateClassification
from dto.UpdateUrgency import UpdateUrgency

# test data
import tests.test_data.RawJSONEntries as RawJSONEntries


def test_DNFUpdateEntry_happy_path():
        test_cases = [
                RawJSONEntries.valid_entry_major_no_severity,
                RawJSONEntries.valid_entry_minor_no_severity,
                RawJSONEntries.valid_entry_patch_no_severity,
                RawJSONEntries.valid_entry_security_no_severity
        ]

        expected_cassification = [
                UpdateClassification.MAJOR,
                UpdateClassification.MINOR,
                UpdateClassification.PATCH,
                UpdateClassification.SECURITY
        ]

        urgency_map = {
                'critical': UpdateUrgency.CRITICAL, 
                'important': UpdateUrgency.IMPORTANT, 
                'moderate': UpdateUrgency.MODERATE, 
                'low': UpdateUrgency.LOW, 
                'none': UpdateUrgency.NONE
        }

        for rawUrgency, enumUrgency in urgency_map.items():
                for index in range(0, 4):
                        input = test_cases[index]
                        input["severity"] = rawUrgency
                        
                        output = DNFUpdateEntry(input)

                        assert(output.key == input["name"])
                        assert(output.packageName == input["nevra"])
                        assert(output.updateType == expected_cassification[index])
                        assert(output.updateUrgency == enumUrgency)

# TODO: mancano i test nel caso la struttura sia differente --> JSON schema validator