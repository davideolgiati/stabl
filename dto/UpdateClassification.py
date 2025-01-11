from enum import Enum
from functools import total_ordering

@total_ordering
class UpdateClassification(Enum):
        SECURITY = 0
        PATCH = 1
        MINOR = 2
        MAJOR = 3

        def __lt__(self, other):
                if self.__class__ is other.__class__:
                        return self.value < other.value
                return NotImplemented