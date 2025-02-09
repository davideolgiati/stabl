# Commands
LIST_UPDATES_CMD = ["dnf", "-C", "updateinfo", "list", "--updates", "--json"]
INSPECT_PKG = lambda pkg: ["rpm", "-q", pkg, "--json"]
GET_INFO_FROM_REPO = lambda pkg: ["dnf", "-C", "repoquery", "--info", pkg, "--quiet"]