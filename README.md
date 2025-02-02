# stabl
A DNF wrapper to selectively choose what packages to upgrade

# v0.0.2
### Features
- Improved version checking system for package updates
- Added functionality to download RPM packages only
- Added package inspection capabilities:
  - Inspect installed packages
  - Inspect downloaded packages

### Bug Fixes
- Added error handling for malformed DNF entry formats
- Added error handling for subprocess failures
- Improved code quality and error handling

# v0.0.1
- initial release
- unit tests

### TODOs:

- better indexing for updates
- use bodhi api to get update info instead of downloading rpms
- add debug option
- class to rapresent partitions
- class to rapresent rpms