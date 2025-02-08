from dto.OrderedEnum import OrderedEnum


class UpdateUrgency(OrderedEnum):
        CRITICAL = 4
        IMPORTANT = 3 
        MODERATE = 2
        LOW = 1 
        NONE = 0