import json
from common.costants import LIST_UPDATES_CMD
from dao.Shell import Shell


def get_update_list_from_repository() -> dict:
        shell: Shell = Shell()
        dnf_output: str = shell.run(LIST_UPDATES_CMD)

        assert isinstance(dnf_output, str)
        assert dnf_output != ''

        json_data: list[dict] = json.loads(dnf_output)

        assert isinstance(json_data, list)
        assert all([isinstance(entry, dict) for entry in json_data])

        assert all(['name'      in entry.keys() for entry in json_data])
        assert all(['type'      in entry.keys() for entry in json_data])
        assert all(['nevra'     in entry.keys() for entry in json_data])
        assert all(['severity'  in entry.keys() for entry in json_data])
        assert all(['buildtime' in entry.keys() for entry in json_data])

        assert all([entry['name'] != ''      for entry in json_data])
        assert all([entry['type'] != ''      for entry in json_data])
        assert all([entry['nevra'] != ''     for entry in json_data])
        assert all([entry['severity'] != ''  for entry in json_data])
        assert all([entry['buildtime'] != '' for entry in json_data])

        assert all([len(entry.keys()) == 5 for entry in json_data])

        return json_data