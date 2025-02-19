import time


class Logger():
        def __new__(cls):
                if not hasattr(cls, 'instance'):
                        cls.instance = super(Logger, cls).__new__(cls)
                        cls.start = None
                return cls.instance

        def info(self, msg, end='\n'):
                assert msg is not None
                assert msg != ""
                print(f"[i] {msg}", end=end, flush=True)

        def debug(self, msg, end='\n'):
                assert msg is not None
                assert msg != ""
                print(f"[*] {msg}", end=end, flush=True)

        def warn(self, msg, end='\n'):
                assert msg is not None
                assert msg != ""
                print(f"[?] {msg}", end=end, flush=True)

        def error(self, msg, end='\n'):
                assert msg is not None
                assert msg != ""
                print(f"[!] {msg}", end=end, flush=True)


def log_timed_execution(description):
        def decorator(func):
                def wrapper(*args, **kwargs):
                        print(f"[i] {description} ... ", end='', flush=True)
                        start = time.time()
                        function_result = func(*args, **kwargs)
                        print(f"done ({time.time() - start:.2f}s)")
                        return function_result
                return wrapper
        return decorator