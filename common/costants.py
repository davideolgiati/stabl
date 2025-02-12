# Commands
LIST_UPDATES_CMD = ["dnf", "-C", "updateinfo", "list", "--updates", "--json"]
INSPECT_PKG = lambda pkg: ["rpm", "-q", pkg, "--json"]
GET_INFO_FROM_REPO = lambda pkg: ["dnf", "-C", "repoquery", "--info", pkg, "--quiet"]
GET_UPDATE_DETAILS = lambda pkgs: ['dnf', '-C', 'repoquery', ' '.join(pkgs), '--quiet' '--queryformat="{\"name\": \"%{name}\", \"version\" : \"%{version}\", \"release\" : \"%{release}\", \"arch\" : \"%{arch}\"},"']