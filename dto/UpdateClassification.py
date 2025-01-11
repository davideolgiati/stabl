from enum import Enum

class UpdateClassification(Enum):
        SECURITY = 0
        PATCH = 1
        MINOR = 2
        MAJOR = 3