# stabl
A DNF wrapper to selectively choose what packages to upgrade

# v0.0.2
- better version check for updates
- add method to download only rpms
- add method to inspect installed pkgs
- add method to inspect downloaded pkgs
- errors on wrong dnf entry format
- errors on subprocess error

# v0.0.1
- initial release
- unit tests

### TODOs:

- manage errors on wrong dnf entry formats
- validation on subprocess output
- validation on dnf update entry
- better indexing for updates
- log what is going on
- use a singleton logger class