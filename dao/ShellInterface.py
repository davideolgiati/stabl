import subprocess

from dto.ManagedShellException import ManagedShellException

class ShellInterface:
        def run(self, command_array):
                try:
                        result = subprocess.run(
                                command_array,
                                stdout=subprocess.PIPE,
                                stderr=subprocess.PIPE,
                        )
                except OSError as e:
                        raise ManagedShellException(
                                str(e),
                                ' '.join(command_array),
                                -1
                        ) from e

                if result.returncode != 0:
                        stderr_msg = result.stderr.decode('utf-8', errors='replace')
                        raise ManagedShellException(
                                stderr_msg.strip(),
                                ' '.join(command_array),
                                result.returncode
                        )

                return result.stdout.decode('utf-8', errors='replace')
        
        def run_unmanaged(self, command_array):
                try:
                        result = subprocess.run(
                                command_array,
                                stdout=subprocess.PIPE,
                                stderr=subprocess.PIPE,
                        )
                except OSError as e:
                        raise ManagedShellException(
                                str(e),
                                ' '.join(command_array),
                                -1
                        ) from e
                
                return {
                        "code": result.returncode,
                        "error": result.stderr.decode('utf-8', errors='replace'),
                        "info": result.stdout.decode('utf-8', errors='replace')
                }
