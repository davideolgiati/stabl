# stabl
A DNF wrapper to selectively choose what packages to upgrade

<img src="./stabl.png" height="900">

# v0.1.3
### Features
 - Implemented a logging system with configurable verbosity levels (trace, debug, info, warn, error) using the `--loglevel` argument.
 - Added a command-line option (`--skip-security-updates`) to allow skipping the inclusion of security updates.

### Enhancements
 - Refactored command-line argument parsing.
 - Updated the internal data model (`Partition`, `Update`, `VersionTag`, `SemanticVersion`, `SecurityClassification`, `ModelBuilder`) for better logic, maintainability, and validation.
 - Improved DNF/RPM command execution logic.
 - Updated console output formatting and added color to the logo.
 - Added unit tests for core components and utilities.

# v0.1.2
### Feature
 - Implemented command-line argument parsing to show help (--help) and to set maximum allowed version bump (major, minor, ptch, release).

### Enhancements
- Refactored command execution logic into a dedicated runner module.
- Removed operating system-specific information display function (display_system_informations).
- Reorganized data model for better maintainability (more in next releases)
- Reorganized dnf and shell modules to simplify logic and implement dependency injection

# v0.1.1
### Features
- Cleaned up code
- Added cli option tho choose update types
- Added cli option to display usage
- Added unit tests for some classes

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