import pytest
from common.logger import Logger

def test_logger_singleton_first_instance():
        # First instance creation
        logger1 = Logger()
        assert isinstance(logger1, Logger)
        assert hasattr(logger1, 'start')
        assert logger1.start is None

def test_logger_singleton_multiple_instances():
        # Create multiple instances
        logger1 = Logger()
        logger2 = Logger()
        logger3 = Logger()
        
        # Verify all instances are the same object
        assert logger1 is logger2
        assert logger2 is logger3
        assert id(logger1) == id(logger2) == id(logger3)

def test_logger_singleton_attribute_persistence():
        # Create first instance and modify attribute
        logger1 = Logger()
        logger1.start = 123
        
        # Create second instance and verify attribute persists
        logger2 = Logger()
        assert logger2.start == 123
        
        # Reset for other tests
        logger1.start = None


def test_info_valid_message(capsys):
        logger = Logger()
        logger.info("Test message")
        captured = capsys.readouterr()
        assert captured.out == "[i] Test message\n"

def test_info_custom_end(capsys):
        logger = Logger()
        logger.info("Test message", end='')
        captured = capsys.readouterr()
        assert captured.out == "[i] Test message"

def test_info_empty_message():
        logger = Logger()
        with pytest.raises(AssertionError):
                logger.info("")

def test_info_none_message():
        logger = Logger()
        with pytest.raises(AssertionError):
                logger.info(None)

def test_debug_valid_message(capsys):
        logger = Logger()
        logger.debug("Test message")
        captured = capsys.readouterr()
        assert captured.out == "[*] Test message\n"

def test_debug_empty_message():
        logger = Logger()
        with pytest.raises(AssertionError):
                logger.debug("")

def test_debug_none_message():
        logger = Logger()
        with pytest.raises(AssertionError):
                logger.debug(None)

def test_warn_valid_message(capsys):
        logger = Logger()
        logger.warn("Test message")
        captured = capsys.readouterr()
        assert captured.out == "[?] Test message\n"

def test_warn_empty_message():
        logger = Logger()
        with pytest.raises(AssertionError):
                logger.warn("")

def test_warn_none_message():
        logger = Logger()
        with pytest.raises(AssertionError):
                logger.warn(None)

def test_error_valid_message(capsys):
        logger = Logger()
        logger.error("Test message")
        captured = capsys.readouterr()
        assert captured.out == "[!] Test message\n"

def test_error_empty_message():
        logger = Logger()
        with pytest.raises(AssertionError):
                logger.error("")

def test_error_none_message():
        logger = Logger()
        with pytest.raises(AssertionError):
                logger.error(None)