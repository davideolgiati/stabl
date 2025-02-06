from dto.OrderedEnum import OrderedEnum


class UpdateClassification(OrderedEnum):
        SECURITY = 0
        RELEASE = 1
        PATCH = 2
        MINOR = 3
        MAJOR = 4