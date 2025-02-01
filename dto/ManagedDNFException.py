import json

class ManagedException(Exception):
    def __init__(self, errors, record):
        assert isinstance(errors, list)
        assert all(isinstance(error, str) for error in errors)
        assert isinstance(record, dict)

        message = f"A Runtime error occurred while running stabl.py:\n {'\n'.join(errors)}\n record: {json.dumps(record)}"            
        
        super().__init__(message)
        self.errors = errors
        self.record = json.dumps(record)
