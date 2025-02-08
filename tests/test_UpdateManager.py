from unittest.mock import patch

from dto.DNFUpdateEntry import DNFUpdateEntry
from dao.DNF import DNF
from dao.UpdateManager import UpdateManager

from tests.unit_tests_utils import mock_shell_run
from tests.test_data.test_valid_updates_partition import expected


@patch("dao.Shell.subprocess.run")
def test_get_data_valid(mock_run):
        mock_run.side_effect = mock_shell_run

        packageManager = DNF()
        updateManager = UpdateManager(packageManager)

        result = updateManager.get_updates_list()
        assert result.keys() == expected.keys()

        for k, v in result.items():
                current_key_packages = expected[k]
                assert len(current_key_packages) == len(v)

                for pkg in v:
                        assert pkg in current_key_packages

        result_2 = updateManager.get_suggested_update_partitions()
        assert result_2 == ["FEDORA-2025-0353c74078", "FEDORA-2025-fb8c11bf7d"]

