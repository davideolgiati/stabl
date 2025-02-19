import json
from common.costants import GET_UPDATE_DETAILS, LIST_UPDATES_CMD
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


def query_packages_repository(signatures) -> list[dict]:
        assert isinstance(signatures, list)
        assert all([ isinstance(signature, str) for signature in signatures ])
        
        if len(signatures) == 0:
                return []

        assert len(signatures) > 0        

        shell: Shell = Shell()
        repository_query: list[str] = GET_UPDATE_DETAILS(signatures)

        assert isinstance(repository_query, list)
        assert repository_query != []
        assert all([isinstance(token, str) for token in repository_query])

        query_result: str = shell.run(repository_query)
        parsed_result = parse_query_result(query_result)

        return parsed_result


def parse_query_result(query_result: str) -> list[dict]:
    assert isinstance(query_result, str)

    json_result: str = f'[{query_result[:-1]}]'

    assert isinstance(json_result, str)
    assert json_result != query_result
    assert json_result != ''

    parsed_result: list[dict] = json.loads(json_result)

    assert isinstance(parsed_result, list)
    for package in parsed_result:
            assert isinstance(package, dict)
            assert len(package.keys()) == 5
                
            assert 'name'      in package.keys()
            assert 'version'   in package.keys()
            assert 'release'   in package.keys()
            assert 'arch'      in package.keys()
            assert 'signature' in package.keys()
                
            assert isinstance(package['name'], str)
            assert isinstance(package['version'], str)
            assert isinstance(package['release'], str)
            assert isinstance(package['arch'], str)
            assert isinstance(package['signature'], list)
            assert len(package['signature']) == 2
            assert isinstance(package['signature'][0], str)
            assert isinstance(package['signature'][1], str)

            assert package['name'] != ''
            assert package['version'] != ''
            assert package['release'] != ''
            assert package['arch'] != ''
            assert package['signature'][0] != ''
            assert package['signature'][1] != ''
            
    return parsed_result