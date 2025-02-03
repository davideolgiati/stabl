from common.costants import GET_SYSTEM_CONFIG, LIST_UPDATES_CMD, DOWNLOAD_UPGRADE
import tests.test_data.test_valid_updates_partition as update_partition_data

from unittest.mock import MagicMock
import copy

base_mock_obj = {
        "returncode": 0,
        "stdout.decode.return_value": "",
        "stderr.decode.return_value": ""
}

DOWNLOAD_UPGRADE_CMD = DOWNLOAD_UPGRADE("")[:-1]

def mock_shell_run(command_array, stdout, stderr):
        mock_stdout = MagicMock()

        if command_array == LIST_UPDATES_CMD:
                return_obj = copy.deepcopy(base_mock_obj)
                return_obj["stdout.decode.return_value"] = update_partition_data.given
                mock_stdout.configure_mock(**return_obj)

        elif command_array == GET_SYSTEM_CONFIG:
                return_obj = copy.deepcopy(base_mock_obj)
                return_obj["stdout.decode.return_value"] = "foo = bar\ncachedir = /var/cache/dnf"
                mock_stdout.configure_mock(**return_obj)

        elif command_array[:-1] == DOWNLOAD_UPGRADE_CMD:
                return_obj = copy.deepcopy(base_mock_obj)
                mock_stdout.configure_mock(**return_obj)

        elif command_array == ["invalid_cmd"]:
                raise OSError(2, "No such file or directory", "invalid_cmd")
        
        elif command_array == ["fake-package"]:
                return_obj = copy.deepcopy(base_mock_obj)
                return_obj["returncode"] = 1
                return_obj["stderr.decode.return_value"] = "Dependency error: Package not found"
                mock_stdout.configure_mock(**return_obj)
        
        else:
                raise NotImplementedError("Command not implemented in mock_run_side_effect")
    
        return mock_stdout