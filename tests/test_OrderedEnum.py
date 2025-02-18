from model.OrderedEnum import OrderedEnum
import pytest

class SampleClass(OrderedEnum):
        ZERO = 0
        ONE = 1

def test_enum_operators():
        zero = SampleClass.ZERO
        one = SampleClass.ONE

        assert((zero > one) == False)
        assert((zero == one) == False)
        assert((zero < one) == True)
        assert((zero <= one) == True)
        assert((zero >= one) == False)
        assert((zero > SampleClass.ZERO) == False)

        with pytest.raises(TypeError):
                assert((zero > "test") == NotImplemented)

