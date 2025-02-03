from unittest.mock import MagicMock, patch
import pytest

from dao.Shell import Shell, ManagedShellException
from common.costants import LIST_UPDATES_CMD

from tests.unit_tests_utils import mock_shell_run
from tests.test_data.test_valid_updates_partition import given


@patch("dao.Shell.subprocess.run")
def test_get_data_valid(mock_run):
    mock_run.side_effect = mock_shell_run

    myShell = Shell()
    result = myShell.run(LIST_UPDATES_CMD)
    
    assert result == given


@patch("dao.Shell.subprocess.run")
def test_run_os_error(mock_run):
    mock_run.side_effect = mock_shell_run

    myShell = Shell()
    
    with pytest.raises(ManagedShellException) as exc_info:
        myShell.run(["invalid_cmd"])
    
    
    assert exc_info.value.return_code == -1
    assert "invalid_cmd" in exc_info.value.shell_cmd
    assert "No such file or directory" in str(exc_info.value.shell_message)
    assert isinstance(exc_info.value.__cause__, OSError)


@patch("dao.Shell.subprocess.run")
def test_run_non_zero_exit(mock_run):
    mock_run.side_effect = mock_shell_run

    myShell = Shell()
    
    with pytest.raises(ManagedShellException) as exc_info:
        myShell.run(["fake-package"])
    
    assert exc_info.value.return_code == 1
    assert "fake-package" in exc_info.value.shell_cmd
    assert "Dependency error" in str(exc_info.value.shell_message)
