import subprocess

from dto.ManagedShellException import ManagedShellException

class ShellInterface:
        def run(self, command_array):
                result = subprocess.run(
                        command_array, 
                        stdout=subprocess.PIPE, 
                        stderr=subprocess.PIPE
                )

                if result.returncode != 0:
                        raise ManagedShellException(
                                result.stderr.decode('utf-8'),
                                ' '.join(command_array),
                                result.returncode
                        )

                return result.stdout.decode('utf-8')