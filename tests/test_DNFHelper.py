from dao.DNFHelper import DNFHelper

from unittest.mock import patch
from tests.unit_tests_utils import mock_shell_run

from tests.test_data.test_valid_updates_partition import expected


@patch("dao.ShellInterface.subprocess.run")
def test_get_data_by_type(mock_run):
        mock_run.side_effect = mock_shell_run

        myDNFHelper = DNFHelper()

        result = myDNFHelper.get_updates_by_partition_id()
        assert result.keys() == expected.keys()

        for k, v in result.items():
                current_key_packages = expected[k]
                assert len(current_key_packages) == len(v)

                for pkg in v:
                        assert pkg in current_key_packages