use criterion::{criterion_group, criterion_main, Criterion};
use stabl::model::version_tag::VersionTag;

fn bench_version_tag() {
        VersionTag::new("137.0","2.fc42");
        VersionTag::new("1.99.2","1744250112.el8");
        VersionTag::new("25.04.0","3.fc42");
        VersionTag::new("257.3","7.fc42");
        VersionTag::new("0^20250415.g2340bbf","1.fc42");
}

pub fn version_tag_benchmark(c: &mut Criterion) {
        c.bench_function("get_updates_list", |b| b.iter(bench_version_tag));
}

criterion_group!(benches,
        version_tag_benchmark
);
criterion_main!(benches);