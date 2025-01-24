from unittest.mock import MagicMock, patch
#import pytest

import tests.test_data.RawDNFOutput as RawDNFOutput

from dao.ShellInterface import ShellInterface
from common.costants import LIST_UPDATES_CMD


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


# @patch("mypackage.proc.subprocess.run", side_effect=Exception("foobar"))
# def test_get_data_invalid(mock_run):
#     with pytest.raises(Exception) as exc:
#         InternalProc.get_data()
#         assert "foobar" in str(exc.value)
