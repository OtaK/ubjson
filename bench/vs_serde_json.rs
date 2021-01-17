use criterion::{Criterion, Throughput, black_box, criterion_group, criterion_main};
use ubjson::parse_one;

const COMPLEX_COUCHDB: &[u8] = include_bytes!("../test/samples/complex/CouchDB4k.ubj");
const COMPLEX_MEDIA: &[u8] = include_bytes!("../test/samples/complex/MediaContent.ubj");
const COMPLEX_TWITTER: &[u8] = include_bytes!("../test/samples/complex/TwitterTimeline.ubj");
const COMPLEX_COUCHDB_JSON: &[u8] = include_bytes!("../test/samples/complex/CouchDB4k.json");
const COMPLEX_MEDIA_JSON: &[u8] = include_bytes!("../test/samples/complex/MediaContent.json");
const COMPLEX_TWITTER_JSON: &[u8] = include_bytes!("../test/samples/complex/TwitterTimeline.json");

pub fn ubjson_vs_serde_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse_complex_couchdb");
    group.throughput(Throughput::Bytes(COMPLEX_COUCHDB.len() as u64));

    group.bench_function("ubjson", |b| {
        b.iter(|| black_box(parse_one(COMPLEX_COUCHDB)))
    });
    group.bench_function("serde_json", |b| {
        b.iter(|| black_box(serde_json::from_slice::<serde_json::Value>(COMPLEX_COUCHDB_JSON)))
    });
    group.finish();

    let mut group = c.benchmark_group("parse_complex_media");
    group.throughput(Throughput::Bytes(COMPLEX_MEDIA.len() as u64));
    group.bench_function("ubjson", |b| {
        b.iter(|| black_box(parse_one(COMPLEX_MEDIA)))
    });
    group.bench_function("serde_json", |b| {
        b.iter(|| black_box(serde_json::from_slice::<serde_json::Value>(COMPLEX_MEDIA_JSON)))
    });
    group.finish();

    let mut group = c.benchmark_group("parse_complex_twitter");
    group.throughput(Throughput::Bytes(COMPLEX_TWITTER.len() as u64));
    group.bench_function("ubjson", |b| {
        b.iter(|| black_box(parse_one(COMPLEX_TWITTER)))
    });
    group.bench_function("serde_json", |b| {
        b.iter(|| black_box(serde_json::from_slice::<serde_json::Value>(COMPLEX_TWITTER_JSON)))
    });
    group.finish();
}

criterion_group!(benches, ubjson_vs_serde_benchmark);
criterion_main!(benches);
