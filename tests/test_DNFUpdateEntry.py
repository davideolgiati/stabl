# classes to be tested 
import pytest
from dto.DNFUpdateEntry import DNFUpdateEntry
from dto.ManagedException import ManagedException
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

def test_DNFUpdateEntry_compare():
        test1 = DNFUpdateEntry(RawJSONEntries.valid_entry_major_no_severity)
        test2 = DNFUpdateEntry(RawJSONEntries.valid_entry_minor_no_severity)
        test3 = DNFUpdateEntry(RawJSONEntries.valid_entry_major_no_severity)

        assert test1 != test2
        assert test1 == test3

        with pytest.raises(TypeError):
                assert("test" != test1)


def test_DNFUpdateEntry_wrong_structure():
        with pytest.raises(ManagedException) as e:
                test1 = DNFUpdateEntry("pippo")
        assert str(e.value) == "A Runtime error occurred while running stabl.py"
        assert e.value.record == '\"pippo\"'
        assert e.value.errors == ["Provided value is not an object"]
        
        with pytest.raises(ManagedException) as e:
                test2 = DNFUpdateEntry({'ciao': 'test'})
        assert str(e.value) == "A Runtime error occurred while running stabl.py"
        assert e.value.record == '{"ciao": "test"}'
        assert e.value.errors == [
                "key 'name' is missing in provided record",
                "key 'nevra' is missing in provided record",
                "key 'type' is missing in provided record",
                "key 'severity' is missing in provided record"
        ]

# TODO: mancano i test nel caso la struttura sia differente --> JSON schema validator