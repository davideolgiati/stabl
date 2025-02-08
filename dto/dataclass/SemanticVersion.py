
class SemanticVersion:
        major: str
        minor: str
        patch: str
        release: str

        @staticmethod
        def fromVersionAndRelease(version, release):
                assert isinstance(version, str)

                version_list = f"{version}.0.0".split('.')

                assert len(version_list) >= 3

                new_object = SemanticVersion()

                new_object.major = version_list[0]
                new_object.minor = version_list[1]
                new_object.patch = '.'.join(version_list[2:])
                new_object.release = release

                return new_object