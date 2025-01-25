from unittest.mock import MagicMock, patch
import pytest  # Ensure pytest is installed/imported
import subprocess

from dao.ShellInterface import ShellInterface, ManagedShellException
from common.costants import LIST_UPDATES_CMD
import tests.test_data.RawDNFOutput as RawDNFOutput

# ---- Existing Test (Valid Case) ----
@patch("dao.ShellInterface.subprocess.run")
def test_get_data_valid(mock_run):
    mock_stdout = MagicMock()
    mock_stdout.configure_mock(
        **{
            "returncode": 0,
            "stdout.decode.return_value": RawDNFOutput.validDNFOutput
        }
    )
    mock_run.return_value = mock_stdout

    myShell = ShellInterface()
    result = myShell.run(LIST_UPDATES_CMD)
    
    assert result == RawDNFOutput.validDNFOutput

# ---- New Test: OSError (e.g., Missing Executable) ----
@patch("dao.ShellInterface.subprocess.run")
def test_run_os_error(mock_run):
    # Simulate a "file not found" error during process startup
    mock_run.side_effect = OSError(2, "No such file or directory", "invalid_cmd")

    myShell = ShellInterface()
    
    with pytest.raises(ManagedShellException) as exc_info:
        myShell.run(["invalid_cmd"])
    
    # Verify exception details
    assert exc_info.value.return_code == -1
    assert "invalid_cmd" in exc_info.value.shell_cmd
    assert "No such file or directory" in str(exc_info.value.shell_message)
    # Ensure original OSError is preserved as the cause
    assert isinstance(exc_info.value.__cause__, OSError)

# ---- New Test: Non-Zero Exit Code ----
@patch("dao.ShellInterface.subprocess.run")
def test_run_non_zero_exit(mock_run):
    mock_result = MagicMock()
    mock_result.configure_mock(
        **{
            "returncode": 1,
            "stderr.decode.return_value": "Dependency error: Package not found"
        }
    )
    mock_run.return_value = mock_result

    myShell = ShellInterface()
    
    with pytest.raises(ManagedShellException) as exc_info:
        myShell.run(["fake-package"])
    
    # Verify exception details
    assert exc_info.value.return_code == 1
    assert "fake-package" in exc_info.value.shell_cmd
    assert "Dependency error" in str(exc_info.value.shell_message)
