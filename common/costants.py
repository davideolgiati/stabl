# Commands
LIST_UPDATES_CMD = ["dnf", "updateinfo", "list", "--updates", "--json"]
DOWNLOAD_UPGRADE = lambda path: ["dnf", "upgrade", "-y", "--downloadonly", f"--destdir={path}"]
INSPECT_PKG = lambda pkg: ["rpm", "-q", pkg, "--json"]
GET_SYSTEM_CONFIG = ["dnf", "--dump-main-config"]
GET_INFO_FROM_REPO = lambda pkg: ["dnf", "--cacheonly", "repoquery", "--info", pkg, "--quiet"]