from dto.enums.OrderedEnum import OrderedEnum


class UpdateUrgency(OrderedEnum):
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
                        'critical':    UpdateUrgency.CRITICAL, 
                        'important':   UpdateUrgency.IMPORTANT, 
                        'moderate':    UpdateUrgency.MODERATE, 
                        'low':         UpdateUrgency.LOW, 
                        'none':        UpdateUrgency.NONE
                }

                key = urgency.lower()
                
                assert key in mapping.keys()
                
                return mapping[key]