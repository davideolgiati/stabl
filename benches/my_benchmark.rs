use criterion::{black_box, criterion_group, criterion_main, Criterion};
use stabl::commons::string::{split_filter_and_deduplicate_string_list, split_string_using_delimiter};

pub fn split_string_repoquery(c: &mut Criterion) {
    c.bench_function("split_string_repoquery", |b| b.iter(|| split_string_using_delimiter(
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
        c.bench_function("split_string_rpm", |b| b.iter(|| split_string_using_delimiter(
            black_box("firefox|#|136.0.1|#|1.fc41"), 
            black_box("|#|")
            )));
}

pub fn split_string_update_info(c: &mut Criterion) {
        c.bench_function("split_string_update_info", |b| b.iter(|| split_string_using_delimiter(
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

criterion_group!(benches, 
        split_string_repoquery, 
        split_dedupe_offset_string_repoquery,
        split_string_rpm, 
        split_string_update_info, 
        split_dedupe_offset_string_update_info
);
criterion_main!(benches);