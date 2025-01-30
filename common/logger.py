import time


class Logger():
        def __new__(cls):
                if not hasattr(cls, 'instance'):
                        cls.instance = super(Logger, cls).__new__(cls)
                return cls.instance
        
        def start_timing(self):
                assert self.start is None
                self.start = time.time()

        def stop_timing(self, msg):
                assert self.start is not None
                print(f"{msg} ({time.time() - self.start:.2f}s)")

                self.start = None

        def info(self, msg, end='\n'):
                assert msg is not None
                assert msg != ""
                print(f"[i] {msg}", end, flush=True)

        def debug(self, msg, end='\n'):
                assert msg is not None
                assert msg != ""
                print(f"[*] {msg}", end, flush=True)

        def warn(self, msg, end='\n'):
                assert msg is not None
                assert msg != ""
                print(f"[?] {msg}", end, flush=True)

        def error(self, msg, end='\n'):
                assert msg is not None
                assert msg != ""
                print(f"[!] {msg}", end, flush=True)