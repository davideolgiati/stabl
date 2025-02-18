import pytest
from model.ManagedDNFException import ManagedException

def test_managed_exception_valid_initialization():
    errors = ["Error 1", "Error 2"]
    record = {"key": "value"}
    exc = ManagedException(errors, record)
    
    assert exc.errors == errors
    assert exc.record == '{"key": "value"}'
    assert isinstance(str(exc), str)
    assert "Error 1" in str(exc)
    assert "Error 2" in str(exc)
    assert '"key": "value"' in str(exc)

def test_managed_exception_empty_valid_inputs():
    exc = ManagedException([], {})
    assert exc.errors == []
    assert exc.record == '{}'

def test_managed_exception_invalid_errors_type():
    with pytest.raises(AssertionError):
        ManagedException("not a list", {})

def test_managed_exception_invalid_record_type():
    with pytest.raises(AssertionError):
        ManagedException([], "not a dict")

def test_managed_exception_none_errors():
    with pytest.raises(AssertionError):
        ManagedException(None, {})

def test_managed_exception_none_record():
    with pytest.raises(AssertionError):
        ManagedException([], None)

def test_managed_exception_message_format():
    errors = ["First error", "Second error"]
    record = {"status": "failed", "code": 500}
    exc = ManagedException(errors, record)
    
    message = str(exc)
    assert "A Runtime error occurred while running stabl.py" in message
    assert "First error" in message
    assert "Second error" in message
    assert '"status": "failed"' in message
    assert '"code": 500' in message

def test_managed_exception_preserves_error_order():
    errors = ["Error 1", "Error 2", "Error 3"]
    exc = ManagedException(errors, {})
    
    message = str(exc)
    first_pos = message.find("Error 1")
    second_pos = message.find("Error 2")
    third_pos = message.find("Error 3")
    
    assert first_pos < second_pos < third_pos