# Commands
LIST_UPDATES_CMD = ["dnf", "updateinfo", "list", "--updates", "--json"]
DOWNLOAD_UPGRADE = lambda pkg: ["dnf", "upgrade", "--downloadonly", "--destdir=/tmp/stabl/", pkg]
INSPECT_PKG = lambda pkg: ["rpm", "-q", pkg, "--json"]