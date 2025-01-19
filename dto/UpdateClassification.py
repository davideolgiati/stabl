from dto.OrderedEnum import OrderedEnum


class UpdateClassification(OrderedEnum):
        SECURITY = 0
        PATCH = 1
        MINOR = 2
        MAJOR = 3