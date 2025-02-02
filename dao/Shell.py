import subprocess

from dto.ManagedShellException import ManagedShellException

class Shell:
        def __new__(cls):
                if not hasattr(cls, 'instance'):
                        cls.instance = super(Shell, cls).__new__(cls)
                        cls.start = None
                return cls.instance

        def run(self, command_array):
                result = self.run_unmanaged(command_array)

                assert isinstance(result, dict)
                assert isinstance(result.get("code"), int)
                assert isinstance(result.get("info"), str)
                assert isinstance(result.get("error"), str)

                if result["code"] != 0:
                        raise ManagedShellException(
                                result["error"].strip(),
                                ' '.join(command_array),
                                result["code"]
                        )

                return result["info"]
        
        def run_unmanaged(self, command_array):
                assert isinstance(command_array, list)
                assert all(isinstance(elem, str) for elem in command_array)

                try:
                        result = subprocess.run(
                                command_array,
                                stdout=subprocess.PIPE,
                                stderr=subprocess.PIPE,
                        )
                except OSError as e:
                        raise ManagedShellException(
                                str(e), ' '.join(command_array), -1
                        ) from e
                
                return {
                        "code": result.returncode,
                        "error": result.stderr.decode('utf-8', errors='replace'),
                        "info": result.stdout.decode('utf-8', errors='replace')
                }
