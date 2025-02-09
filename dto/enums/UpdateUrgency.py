from dto.enums.OrderedEnum import OrderedEnum


class UpdateUrgency(OrderedEnum):
        CRITICAL = 4
        IMPORTANT = 3 
        MODERATE = 2
        LOW = 1 
        NONE = 0

        @staticmethod
        def fromString(urgency_id):
                update_urgency_mapping = {
                        'critical':    UpdateUrgency.CRITICAL, 
                        'important':   UpdateUrgency.IMPORTANT, 
                        'moderate':    UpdateUrgency.MODERATE, 
                        'low':         UpdateUrgency.LOW, 
                        'none':        UpdateUrgency.NONE
                }

                return update_urgency_mapping[urgency_id]