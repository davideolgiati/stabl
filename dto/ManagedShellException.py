
class ManagedShellException(Exception):
    def __init__(self, shell_message, shell_cmd, return_code):            
        super().__init__("A Runtime error occurred while running stabl.py")
        self.shell_message = shell_message
        self.shell_cmd = shell_cmd
        self.return_code = return_code
