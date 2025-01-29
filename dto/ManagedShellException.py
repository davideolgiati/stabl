
class ManagedShellException(Exception):
    def __init__(self, shell_message, shell_cmd, return_code):
        assert isinstance(shell_message, str)
        assert isinstance(shell_cmd, str)
        assert all(isinstance(token, str) for token in shell_cmd)
        assert isinstance(return_code, int)
        
        message = f"the command \"{' '.join(shell_cmd)}\" returned {return_code}\n{shell_message}"

        super().__init__(message)
        self.shell_message = shell_message
        self.shell_cmd = shell_cmd
        self.return_code = return_code
