import json


class ManagedException(Exception):
    def __init__(self, errors, record):            
        super().__init__("A Runtime error occurred while running stabl.py")
        self.errors = errors
        self.record = json.dumps(record)
