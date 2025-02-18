from model.enum.OrderedEnum import OrderedEnum


class SecurityClass(OrderedEnum):
        CRITICAL = 4
        IMPORTANT = 3 
        MODERATE = 2
        LOW = 1 
        NONE = 0

        @staticmethod
        def fromString(urgency):
                assert isinstance(urgency, str)
                assert urgency != ''
                
                mapping = {
                        'critical':    SecurityClass.CRITICAL, 
                        'important':   SecurityClass.IMPORTANT, 
                        'moderate':    SecurityClass.MODERATE, 
                        'low':         SecurityClass.LOW, 
                        'none':        SecurityClass.NONE
                }

                key = urgency.lower()
                
                assert key in mapping.keys()
                
                return mapping[key]