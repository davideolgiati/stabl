from dto.OrderedEnum import OrderedEnum


class UpdateUrgency(OrderedEnum):
        CRITICAL = 0
        IMPORTANT = 1 
        MODERATE = 2
        LOW = 3 
        NONE = 4