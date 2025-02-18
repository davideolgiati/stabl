from model.DNFUpdateEntry import DNFUpdateEntry


given = """
[
  {
    "name":"FEDORA-2025-0353c74078",
    "type":"security",
    "severity":"None",
    "nevra":"xorg-x11-xinit-1.4.3-1.fc41.x86_64",
    "buildtime":"2025-01-16 01:58:44"
  },
  {
    "name":"FEDORA-2025-0353c74078",
    "type":"bugfix",
    "severity":"None",
    "nevra":"xxd-2:9.1.1000-1.fc41.x86_64",
    "buildtime":"2025-01-12 01:37:12"
  },
  {
    "name":"FEDORA-2025-5c56962500",
    "type":"enhancement",
    "severity":"None",
    "nevra":"xxhash-libs-0.8.3-1.fc41.x86_64",
    "buildtime":"2025-01-07 02:44:33"
  },
  {
    "name":"FEDORA-2025-fb8c11bf7d",
    "type":"unspecified",
    "severity":"None",
    "nevra":"zlib-ng-compat-2.2.3-1.fc41.x86_64",
    "buildtime":"2025-01-16 01:58:44"
  },
  {
    "name":"FEDORA-2025-fb8c11bf7d",
    "type":"unspecified",
    "severity":"moderate",
    "nevra":"zlib-ng-compat-2.2.3-1.fc41.i686",
    "buildtime":"2025-01-16 01:58:44"
  }
]
"""

expected = {
  "FEDORA-2025-0353c74078" : [
    DNFUpdateEntry({
      "name":"FEDORA-2025-0353c74078",
      "type":"security",
      "severity":"None",
      "nevra":"xorg-x11-xinit-1.4.3-1.fc41.x86_64",
      "buildtime":"2025-01-16 01:58:44"
    }),
          DNFUpdateEntry({
                  "name":"FEDORA-2025-0353c74078",
                  "type":"bugfix",
                  "severity":"None",
                  "nevra":"xxd-2:9.1.1000-1.fc41.x86_64",
                  "buildtime":"2025-01-12 01:37:12"
          })
  ],
        "FEDORA-2025-5c56962500" : [
                DNFUpdateEntry({
                        "name":"FEDORA-2025-5c56962500",
                        "type":"enhancement",
                        "severity":"None",
                        "nevra":"xxhash-libs-0.8.3-1.fc41.x86_64",
                        "buildtime":"2025-01-07 02:44:33"
                })
        ],
        "FEDORA-2025-fb8c11bf7d" : [
                DNFUpdateEntry({
                        "name":"FEDORA-2025-fb8c11bf7d",
                        "type":"unspecified",
                        "severity":"None",
                        "nevra":"zlib-ng-compat-2.2.3-1.fc41.x86_64",
                        "buildtime":"2025-01-16 01:58:44"
                }),
                DNFUpdateEntry({
                        "name":"FEDORA-2025-fb8c11bf7d",
                        "type":"unspecified",
                        "severity":"moderate",
                        "nevra":"zlib-ng-compat-2.2.3-1.fc41.i686",
                        "buildtime":"2025-01-16 01:58:44"
                })
        ]
}
