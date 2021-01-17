use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ubjson::parse_one;

const COMPLEX_COUCHDB: &[u8] = include_bytes!("../test/samples/complex/CouchDB4k.ubj");
const COMPLEX_MEDIA: &[u8] = include_bytes!("../test/samples/complex/MediaContent.ubj");
const COMPLEX_TWITTER: &[u8] = include_bytes!("../test/samples/complex/TwitterTimeline.ubj");

pub fn ubjson_benchmark(c: &mut Criterion) {
    c.bench_function("parse_complex_couchdb", |b| {
        b.iter(|| black_box(parse_one(COMPLEX_COUCHDB)))
    });

    c.bench_function("parse_complex_media", |b| {
        b.iter(|| black_box(parse_one(COMPLEX_MEDIA)))
    });

    c.bench_function("parse_complex_twitter", |b| {
        b.iter(|| black_box(parse_one(COMPLEX_TWITTER)))
    });
}

criterion_group!(benches, ubjson_benchmark);
criterion_main!(benches);
