use criterion::{black_box, criterion_group, criterion_main, Criterion};
use stabl::{commons::string::{split_filter_and_deduplicate_string_list, split_string}, system::shell_cmd_facade::get_updateinfo_output};

pub fn split_string_repoquery(c: &mut Criterion) {
    c.bench_function("split_string_repoquery", |b| b.iter(|| split_string(
        black_box("firefox|#|131.0.2|#|1.fc41|#|firefox-0:131.0.2-1.fc41.x86_64|#|firefox-131.0.2-1.fc41.x86_64"), 
        black_box("|#|")
        )));
}

pub fn split_dedupe_offset_string_repoquery(c: &mut Criterion) {
        c.bench_function("split_dedupe_offset_string_repoquery", |b| b.iter(|| split_filter_and_deduplicate_string_list(
                black_box(&["firefox|#|131.0.2|#|1.fc41|#|firefox-0:131.0.2-1.fc41.x86_64|#|firefox-131.0.2-1.fc41.x86_64"]), 
            black_box("|#|"),
            black_box(0)
            )));
}

pub fn split_string_rpm(c: &mut Criterion) {
        c.bench_function("split_string_rpm", |b| b.iter(|| split_string(
            black_box("firefox|#|136.0.1|#|1.fc41"), 
            black_box("|#|")
            )));
}

pub fn split_string_update_info(c: &mut Criterion) {
        c.bench_function("split_string_update_info", |b| b.iter(|| split_string(
            black_box("FEDORA-2025-f14b0ee7be enhancement None                           firefox-131.0.2-1.fc41.x86_64 2025-03-17 01:37:24"), 
            black_box(" ")
            )));
}

pub fn split_dedupe_offset_string_update_info(c: &mut Criterion) {
        c.bench_function("split_dedupe_offset_string_update_info", |b| b.iter(|| split_filter_and_deduplicate_string_list(
            black_box(&["FEDORA-2025-f14b0ee7be enhancement None                           firefox-131.0.2-1.fc41.x86_64 2025-03-17 01:37:24"]), 
            black_box(" "),
            black_box(3)
        )));
}

fn dnf_updatelist_mock(_arg1: &str, _arg2: &[String]) -> String {
        String::from(
"Name                   Type        Severity                                            Package              Issued
FEDORA-2025-1a03bbb363 bugfix      None                         gutenprint-5.3.5-3.fc41.x86_64 2025-03-20 06:31:13
FEDORA-2025-1a03bbb363 bugfix      None                    gutenprint-cups-5.3.5-3.fc41.x86_64 2025-03-20 06:31:13
FEDORA-2025-1a03bbb363 bugfix      None                    gutenprint-libs-5.3.5-3.fc41.x86_64 2025-03-20 06:31:13
FEDORA-2025-1a0c45a564 enhancement None                      vim-data-2:9.1.1227-1.fc41.noarch 2025-03-23 01:13:07
FEDORA-2025-1a0c45a564 enhancement None                   vim-minimal-2:9.1.1227-1.fc41.x86_64 2025-03-23 01:13:07
FEDORA-2025-1a0c45a564 enhancement None                           xxd-2:9.1.1227-1.fc41.x86_64 2025-03-23 01:13:07
FEDORA-2025-227a3afc1f enhancement None                             dpkg-1.22.15-1.fc41.x86_64 2025-03-21 01:13:34
FEDORA-2025-227a3afc1f enhancement None                         dpkg-dev-1.22.15-1.fc41.noarch 2025-03-21 01:13:34
FEDORA-2025-227a3afc1f enhancement None                        dpkg-perl-1.22.15-1.fc41.noarch 2025-03-21 01:13:34
FEDORA-2025-3bef9ab047 bugfix      None                     cups-filters-1:2.0.1-3.fc41.x86_64 2025-03-19 02:30:50
FEDORA-2025-3bef9ab047 bugfix      None          cups-filters-driverless-1:2.0.1-3.fc41.x86_64 2025-03-19 02:30:50
FEDORA-2025-4a834c2b43 bugfix      None                          kernel-6.13.8-200.fc41.x86_64 2025-03-24 02:18:39
FEDORA-2025-4a834c2b43 bugfix      None                     kernel-core-6.13.8-200.fc41.x86_64 2025-03-24 02:18:39
FEDORA-2025-4a834c2b43 bugfix      None                  kernel-modules-6.13.8-200.fc41.x86_64 2025-03-24 02:18:39
FEDORA-2025-4a834c2b43 bugfix      None             kernel-modules-core-6.13.8-200.fc41.x86_64 2025-03-24 02:18:39
FEDORA-2025-4a834c2b43 bugfix      None            kernel-modules-extra-6.13.8-200.fc41.x86_64 2025-03-24 02:18:39
FEDORA-2025-4a834c2b43 bugfix      None                    kernel-tools-6.13.8-200.fc41.x86_64 2025-03-24 02:18:39
FEDORA-2025-4a834c2b43 bugfix      None               kernel-tools-libs-6.13.8-200.fc41.x86_64 2025-03-24 02:18:39
FEDORA-2025-4a834c2b43 bugfix      None                    python3-perf-6.13.8-200.fc41.x86_64 2025-03-24 02:18:39
FEDORA-2025-4cd6805b63 enhancement None                        hwloc-libs-2.12.0-1.fc41.x86_64 2025-03-20 04:35:20
FEDORA-2025-50dcfe3c49 bugfix      Moderate            passt-0^20250320.g32f6212-2.fc41.x86_64 2025-03-23 01:13:07
FEDORA-2025-50dcfe3c49 bugfix      Moderate    passt-selinux-0^20250320.g32f6212-2.fc41.noarch 2025-03-23 01:13:07
FEDORA-2025-5d959bdf1d enhancement None                         libfprint-1.94.9-1.fc41.x86_64 2025-03-22 04:09:56
FEDORA-2025-68a042226c enhancement None              container-selinux-4:2.236.0-1.fc41.noarch 2025-03-20 04:35:20
FEDORA-2025-73800111e8 security    Moderate                       ipp-usb-0.9.30-1.fc41.x86_64 2025-03-19 02:30:50
FEDORA-2025-7755eec1cb unspecified None                  python3-regex-2024.11.6-1.fc41.x86_64 2025-03-12 02:01:22
FEDORA-2025-80fa816815 enhancement None                          git-core-2.49.0-1.fc41.x86_64 2025-03-20 04:35:20
FEDORA-2025-8c1ad2afb5 enhancement None                 breeze-icon-theme-6.12.0-1.fc41.noarch 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                        kf6-attica-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                         kf6-baloo-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                    kf6-baloo-file-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                    kf6-baloo-libs-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                      kf6-bluez-qt-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                  kf6-breeze-icons-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                    kf6-filesystem-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None          kf6-frameworkintegration-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None     kf6-frameworkintegration-libs-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                      kf6-karchive-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                         kf6-kauth-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                    kf6-kbookmarks-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                 kf6-kcalendarcore-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                      kf6-kcmutils-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                       kf6-kcodecs-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                  kf6-kcolorscheme-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                   kf6-kcompletion-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                       kf6-kconfig-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                kf6-kconfigwidgets-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                     kf6-kcontacts-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                   kf6-kcoreaddons-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                        kf6-kcrash-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                   kf6-kdbusaddons-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                  kf6-kdeclarative-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                          kf6-kded-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                         kf6-kdesu-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                        kf6-kdnssd-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                     kf6-kdoctools-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                 kf6-kfilemetadata-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                  kf6-kglobalaccel-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                    kf6-kguiaddons-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                     kf6-kholidays-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                         kf6-ki18n-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                   kf6-kiconthemes-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                     kf6-kidletime-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                 kf6-kidletime-x11-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                 kf6-kimageformats-6.12.0-2.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                      kf6-kio-core-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                 kf6-kio-core-libs-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                       kf6-kio-doc-6.12.0-1.fc41.noarch 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None              kf6-kio-file-widgets-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                       kf6-kio-gui-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                   kf6-kio-widgets-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None              kf6-kio-widgets-libs-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                      kf6-kirigami-6.12.0-2.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                   kf6-kitemmodels-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                    kf6-kitemviews-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                   kf6-kjobwidgets-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                     kf6-knewstuff-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                kf6-knotifications-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                 kf6-knotifyconfig-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                      kf6-kpackage-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                        kf6-kparts-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                       kf6-kpeople-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                          kf6-kpty-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                  kf6-kquickcharts-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                       kf6-krunner-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                      kf6-kservice-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None           kf6-kstatusnotifieritem-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                          kf6-ksvg-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                   kf6-ktexteditor-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                 kf6-ktexttemplate-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                  kf6-ktextwidgets-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None               kf6-kunitconversion-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                 kf6-kuserfeedback-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                       kf6-kwallet-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                  kf6-kwallet-libs-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                kf6-kwidgetsaddons-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                 kf6-kwindowsystem-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                       kf6-kxmlgui-6.12.0-2.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None               kf6-modemmanager-qt-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None             kf6-networkmanager-qt-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                        kf6-prison-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                       kf6-purpose-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None            kf6-qqc2-desktop-style-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                         kf6-solid-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                        kf6-sonnet-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None               kf6-sonnet-hunspell-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                   kf6-syndication-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None           kf6-syntax-highlighting-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-8c1ad2afb5 enhancement None                  kf6-threadweaver-6.12.0-1.fc41.x86_64 2025-03-15 02:23:06
FEDORA-2025-a2221463a6 enhancement None                              nspr-4.36.0-4.fc41.x86_64 2025-03-13 01:09:59
FEDORA-2025-a2221463a6 enhancement None                              nss-3.109.0-1.fc41.x86_64 2025-03-13 01:09:59
FEDORA-2025-a2221463a6 enhancement None                      nss-softokn-3.109.0-1.fc41.x86_64 2025-03-13 01:09:59
FEDORA-2025-a2221463a6 enhancement None               nss-softokn-freebl-3.109.0-1.fc41.x86_64 2025-03-13 01:09:59
FEDORA-2025-a2221463a6 enhancement None                      nss-sysinit-3.109.0-1.fc41.x86_64 2025-03-13 01:09:59
FEDORA-2025-a2221463a6 enhancement None                        nss-tools-3.109.0-1.fc41.x86_64 2025-03-13 01:09:59
FEDORA-2025-a2221463a6 enhancement None                         nss-util-3.109.0-1.fc41.x86_64 2025-03-13 01:09:59
FEDORA-2025-f14b0ee7be enhancement None                python3-argcomplete-3.6.0-1.fc41.noarch 2025-03-17 01:37:24
FEDORA-2025-f455f56914 bugfix      None                     blivet-data-1:3.11.0-4.fc41.noarch 2025-03-22 01:49:49
FEDORA-2025-f455f56914 bugfix      None                  python3-blivet-1:3.11.0-4.fc41.noarch 2025-03-22 01:49:49"
)
}

pub fn get_updates_list_benchmark(c: &mut Criterion) {
        c.bench_function("get_updates_list", |b| b.iter(|| get_updateinfo_output(dnf_updatelist_mock)));
}

criterion_group!(benches, 
        split_string_repoquery, 
        split_dedupe_offset_string_repoquery,
        split_string_rpm, 
        split_string_update_info, 
        split_dedupe_offset_string_update_info,
        get_updates_list_benchmark
);
criterion_main!(benches);