# classes to be tested 
import pytest
from model.DNFUpdateEntry import DNFUpdateEntry
from model.ManagedDNFException import ManagedException
from model.UpdateClassification import UpdateClassification
from model.UpdateUrgency import UpdateUrgency

# test data
import tests.test_data.sample_json_dnf_update_output as sample_json_dnf_update_output


def test_DNFUpdateEntry_happy_path():
        test_cases = [
                sample_json_dnf_update_output.valid_entry_major_no_severity,
                sample_json_dnf_update_output.valid_entry_minor_no_severity,
                sample_json_dnf_update_output.valid_entry_patch_no_severity,
                sample_json_dnf_update_output.valid_entry_security_no_severity
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
        test1 = DNFUpdateEntry(sample_json_dnf_update_output.valid_entry_major_no_severity)
        test2 = DNFUpdateEntry(sample_json_dnf_update_output.valid_entry_minor_no_severity)
        test3 = DNFUpdateEntry(sample_json_dnf_update_output.valid_entry_major_no_severity)

        assert test1 != test2
        assert test1 == test3

        with pytest.raises(TypeError):
                assert("test" != test1)


def test_DNFUpdateEntry_wrong_structure():
        with pytest.raises(AssertionError) as e:
                test1 = DNFUpdateEntry("pippo")
        
        with pytest.raises(AssertionError) as e:
                test2 = DNFUpdateEntry({'ciao': 'test'})
