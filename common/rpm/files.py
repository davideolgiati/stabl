import re

from common import regex


def is_valid_rpm_file_path(path):
        assert(isinstance(path, str))

        if re.search(regex.valid_rpm_file, path, re.IGNORECASE):
                return True
        else:
                return False

def is_file_rpm(path):
        assert(isinstance(path, str))
        assert(path != "")

        rpm_magic_bytes = b'\xed\xab\xee\xdb'
        with open(path, 'rb') as fp:
                file_magic_bytes = fp.read(4)

        magic_bytes_check = file_magic_bytes == rpm_magic_bytes
        assert isinstance(magic_bytes_check, bool)

        return magic_bytes_check