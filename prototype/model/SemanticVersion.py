
class SemanticVersion:
        major: str
        minor: str
        patch: str
        release: str

        @staticmethod
        def fromVersionAndRelease(version, release):
                assert isinstance(version, str)

                version_list = f"{version}.0.0".split('.')

                while len(version_list) > 3 and version_list[-1] == "0":
                        version_list.pop()

                assert len(version_list) >= 3

                result = SemanticVersion()

                result.major = version_list[0]
                result.minor = version_list[1]
                result.patch = '.'.join(version_list[2:])
                result.release = release

                return result