from dao.DNF import DNF

from unittest.mock import patch
from tests.unit_tests_utils import mock_shell_run

from tests.test_data.test_valid_updates_partition import expected


@patch("dao.Shell.subprocess.run")
def test_get_data_by_type(mock_run):
        mock_run.side_effect = mock_shell_run

        myDNFHelper = DNF()

        result = myDNFHelper.get_update_partitions()
        assert result.keys() == expected.keys()

        for k, v in result.items():
                current_key_packages = expected[k]
                assert len(current_key_packages) == len(v)

                for pkg in v:
                        assert pkg in current_key_packages