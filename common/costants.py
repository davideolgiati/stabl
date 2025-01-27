# Commands
LIST_UPDATES_CMD = ["dnf", "updateinfo", "list", "--updates", "--json"]
DOWNLOAD_UPGRADE = ["dnf", "upgrade", "-y", "--downloadonly", "--destdir=/tmp/stabl/"]
INSPECT_PKG = lambda pkg: ["rpm", "-q", pkg, "--json"]