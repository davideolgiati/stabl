import json
import re

from common import regex
from common.costants import GET_INFO_FROM_REPO, INSPECT_PKG
from dao.Shell import Shell
from model.SemanticVersion import SemanticVersion


def format_package_version(version, release):
        assert isinstance(version, str)
        assert isinstance(release, str)
        assert version != ""

        tokenized_version = re.split(regex.valid_separator, version)

        final_version = version
        final_release = release

        if(len(tokenized_version) > 1):
                final_version = tokenized_version[0]
                additional_info = ''.join(tokenized_version[1:])
                final_release += f"-{additional_info}"


        assert re.findall(regex.package_version, final_version) != []

        return {"version": final_version, "release": final_release}

def process_rpm_json_output(string):
        stage_1 = string.replace("}\n{", "},\n{")
        stage_2 = f"[{stage_1}]"
        rpms_properties_list = json.loads(stage_2)

        rpm_sort_function = lambda rpm: rpm["Buildtime"]
        rpms_properties_list.sort(key=rpm_sort_function)
        rpm_properties = rpms_properties_list[-1]

        assert isinstance(rpm_properties, dict)
        for key in ["Name", "Version", "Release", "Arch"]:
                assert isinstance(rpm_properties.get(key), str)
                assert rpm_properties.get(key) != ""

        assert re.findall(regex.package_name, rpm_properties["Name"]) != []

        return rpm_properties

def process_repoquery_output(string):
        stage_1 = string.split('\n')
        stage_2 = [line.split(':') for line in stage_1 if ':' in line]
        stage_3 = {entry[0].strip(): ':'.join(entry[1:]).strip() for entry in stage_2}

        rpm_properties = {
               "Name": stage_3["Name"], 
               "Version": stage_3["Version"], 
               "Release": stage_3["Release"], 
               "Arch": stage_3["Architecture"]
        }

        assert isinstance(rpm_properties, dict)
        for key in ["Name", "Version", "Release", "Arch"]:
                assert isinstance(rpm_properties.get(key), str)
                assert rpm_properties.get(key) != ""

        assert re.findall(regex.package_name, rpm_properties["Name"]) != []

        return rpm_properties

def run_rpm_query_command(package_signature):
        shell = Shell()

        inspect_command = INSPECT_PKG(package_signature)
        shell_outcome = shell.run_unmanaged(inspect_command)
        assert isinstance(shell_outcome, dict)
        assert isinstance(shell_outcome.get("code"), int)
        assert isinstance(shell_outcome.get("info"), str)

        return_code = shell_outcome["code"]
        stdout_message = shell_outcome["info"]

        if(return_code != 0):
                raise ValueError
        
        return stdout_message

def run_dnf_repoquery_command(package_signature):
        shell = Shell()

        inspect_command = GET_INFO_FROM_REPO(package_signature)
        shell_outcome = shell.run_unmanaged(inspect_command)
        assert isinstance(shell_outcome, dict)
        assert isinstance(shell_outcome.get("code"), int)
        assert isinstance(shell_outcome.get("info"), str)

        return_code = shell_outcome["code"]
        stdout_message = shell_outcome["info"]

        if(return_code != 0):
                raise ValueError
        
        return stdout_message

def query_installed_package_info(package_reference):
        assert(isinstance(package_reference, str))
        assert(package_reference != "")
        
        stdout_message = run_rpm_query_command(package_reference)
        rpm_properties = process_rpm_json_output(stdout_message)

        return generate_semantic_version_from_rpm_properties(rpm_properties)


def query_package_info_from_signature(package_signature):
        assert(isinstance(package_signature, str))
        assert(package_signature != "")

        stdout_message = run_dnf_repoquery_command(package_signature)
        if stdout_message == '':
                raise KeyError
        
        rpm_properties = process_repoquery_output(stdout_message)
        package_name = rpm_properties["Name"]
        semantic_version = generate_semantic_version_from_rpm_properties(rpm_properties)

        return package_name, semantic_version


def generate_semantic_version_from_rpm_properties(rpm_properties):
        rpm_version = rpm_properties["Version"]
        rpm_release = rpm_properties["Release"]

        final_version = format_package_version(rpm_version, rpm_release)
        assert isinstance(final_version, dict)
        assert isinstance(final_version.get("version"), str)
        assert isinstance(final_version.get("release"), str)

        return SemanticVersion.fromVersionAndRelease(
                final_version["version"],
                final_version["release"]
        )