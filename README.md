# stabl
A DNF wrapper to selectively choose what packages to upgrade

# v0.1.0
### Features
- First real working version
- Rust Rewrite

# v0.0.5
### Features
- Enhanced repository query system to include update urgency and partition information
- Improved partition computation logic to consider update urgency for recommendations

# v0.0.4
### Features
- Improved clarity and consistency in package handling
- Refactored DNF, UpdateManager, and Shell classes to streamline data handling, command execution, and maintainability
- Enhanced package signature validation and processing for more secure operations
- Improved command execution logic in Shell and DNF classes to reduce redundancy and errors
- Simplified update details parsing and strengthened error handling across core modules

# v0.0.3
### Features
- Improved version checking system using semantic versioning via the SemanticVersion class
- Refactored DNF, RPM, and UpdateManager classes for better code clarity and maintainability
- Enhanced RPM class with robust version parsing and package query logic
- Implemented a singleton pattern in the Shell class to ensure consistent shell interactions
- Optimized repoquery command usage for cached queries and reduced redundancy
- Removed unused RPM-related files and simplified update handling logic

### Bug Fixes
- Added comprehensive error handling and exception management improvements
- Fixed package name validation logic to prevent incorrect inputs

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
- add debug option