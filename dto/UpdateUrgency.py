from enum import Enum
from functools import total_ordering

@total_ordering
class UpdateUrgency(Enum):
        CRITICAL = 0
        IMPORTANT = 1 
        MODERATE = 2
        LOW = 3 
        NONE = 4

        def __lt__(self, other):
                if self.__class__ is other.__class__:
                        return self.value < other.value
                return NotImplemented