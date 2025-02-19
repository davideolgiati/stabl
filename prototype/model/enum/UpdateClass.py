from model.enum.OrderedEnum import OrderedEnum


class UpdateClass(OrderedEnum):
        SECURITY = 0
        RELEASE = 1
        PATCH = 2
        MINOR = 3
        MAJOR = 4