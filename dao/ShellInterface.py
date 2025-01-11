import subprocess

class ShellInterface:
        def run(self, command_array):
                result = subprocess.run(command_array, stdout=subprocess.PIPE)
                return result.stdout.decode('utf-8')