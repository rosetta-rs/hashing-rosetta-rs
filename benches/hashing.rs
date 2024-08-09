use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::hash::Hasher;

fn benchmark_short(c: &mut Criterion) {
    let mut group = c.benchmark_group("short article");

    macro_rules! generate_functions {
        ($name: ident, $hash_fn: expr) => {{
            group.bench_function(stringify!($name), |b| b.iter(|| short()));
            fn short() -> bool {
                let a = $hash_fn(black_box(include_bytes!("../article1.txt")));
                let b = $hash_fn(black_box(include_bytes!("../article2.txt")));

                // False
                a == b
            }
        }};
    }

    generate_functions!(xxh3, twox_hash::xxh3::hash64);
    generate_functions!(meowhash, meowhash::MeowHasher::hash);
    generate_functions!(ahash, __hash::<::ahash::AHasher>);
    generate_functions!(fasthash, fasthash::metro::hash64);
    generate_functions!(default_hasher, __hash::<std::hash::DefaultHasher>);
    generate_functions!(rustc_hash, __hash::<::rustc_hash::FxHasher>);
    generate_functions!(fnv, __hash::<::fnv::FnvHasher>);
    generate_functions!(xxhash_rust_xxh3, __hash::<::xxhash_rust::xxh3::Xxh3>);
    generate_functions!(highway, __hash::<::highway::HighwayHasher>);
    generate_functions!(cityhasher, __hash::<::cityhasher::CityHasher>);
    generate_functions!(gxhash, __hash::<::gxhash::GxHasher>);
    generate_functions!(wyhash, __hash::<::wyhash::WyHash>);
    generate_functions!(blake3, ::blake3::hash);
    generate_functions!(
        hud_slice_by_8,
        __hash::<::hud_slice_by_8::crc32::CRC32Hasher>
    );
    group.finish();
}

fn benchmark_long(c: &mut Criterion) {
    let mut group = c.benchmark_group("long article");

    macro_rules! generate_functions {
        ($name: ident, $hash_fn: expr) => {{
            group.bench_function(stringify!($name), |b| b.iter(|| long()));
            fn long() -> bool {
                let a = $hash_fn(black_box(include_bytes!("../article3.txt")));
                let b = $hash_fn(black_box(include_bytes!("../article4.txt")));

                // False
                a == b
            }
        }};
    }

    generate_functions!(xxh3, twox_hash::xxh3::hash64);
    generate_functions!(meowhash, meowhash::MeowHasher::hash);
    generate_functions!(ahash, __hash::<::ahash::AHasher>);
    generate_functions!(fasthash, fasthash::metro::hash64);
    generate_functions!(default_hasher, __hash::<std::hash::DefaultHasher>);
    generate_functions!(rustc_hash, __hash::<::rustc_hash::FxHasher>);
    generate_functions!(fnv, __hash::<::fnv::FnvHasher>);
    generate_functions!(xxhash_rust_xxh3, __hash::<::xxhash_rust::xxh3::Xxh3>);
    generate_functions!(highway, __hash::<::highway::HighwayHasher>);
    generate_functions!(cityhasher, __hash::<::cityhasher::CityHasher>);
    generate_functions!(gxhash, __hash::<::gxhash::GxHasher>);
    generate_functions!(wyhash, __hash::<::wyhash::WyHash>);
    generate_functions!(blake3, ::blake3::hash);
    generate_functions!(
        hud_slice_by_8,
        __hash::<::hud_slice_by_8::crc32::CRC32Hasher>
    );
}

fn benchmark_equal(c: &mut Criterion) {
    let mut group = c.benchmark_group("long article");

    macro_rules! generate_functions {
        ($name: ident, $hash_fn: expr) => {{
            group.bench_function(stringify!($name), |b| b.iter(|| equal()));
            fn equal() -> bool {
                let a = $hash_fn(black_box(include_bytes!("../article4.txt")));
                let b = $hash_fn(black_box(include_bytes!("../article5.txt")));

                // True
                a == b
            }
        }};
    }

    generate_functions!(xxh3, twox_hash::xxh3::hash64);
    generate_functions!(meowhash, meowhash::MeowHasher::hash);
    generate_functions!(ahash, __hash::<::ahash::AHasher>);
    generate_functions!(fasthash, fasthash::metro::hash64);
    generate_functions!(default_hasher, __hash::<std::hash::DefaultHasher>);
    generate_functions!(rustc_hash, __hash::<::rustc_hash::FxHasher>);
    generate_functions!(fnv, __hash::<::fnv::FnvHasher>);
    generate_functions!(xxhash_rust_xxh3, __hash::<::xxhash_rust::xxh3::Xxh3>);
    generate_functions!(highway, __hash::<::highway::HighwayHasher>);
    generate_functions!(cityhasher, __hash::<::cityhasher::CityHasher>);
    generate_functions!(gxhash, __hash::<::gxhash::GxHasher>);
    generate_functions!(wyhash, __hash::<::wyhash::WyHash>);
    generate_functions!(blake3, ::blake3::hash);
    generate_functions!(
        hud_slice_by_8,
        __hash::<::hud_slice_by_8::crc32::CRC32Hasher>
    );
}

fn __hash<H: Hasher + Default>(input: &[u8]) -> u64 {
    let mut hasher = H::default();
    hasher.write(input);
    hasher.finish()
}

fn bench_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("== comparsion");
    group.bench_function("short", |b| b.iter(|| comparison_short()));
    group.bench_function("long", |b| b.iter(|| comparison_long()));
    group.bench_function("true", |b| b.iter(|| comparison_true()));

    fn comparison_short() -> bool {
        let a = black_box(include_str!("../article1.txt"));
        let b = black_box(include_str!("../article2.txt"));

        // False
        a == b
    }

    fn comparison_long() -> bool {
        let a = black_box(include_str!("../article3.txt"));
        let b = black_box(include_str!("../article4.txt"));

        // False
        a == b
    }

    fn comparison_true() -> bool {
        let a = black_box(include_str!("../article4.txt"));
        let b = black_box(include_str!("../article5.txt"));

        // True
        a == b
    }
}

criterion_group!(
    benches,
    benchmark_long,
    benchmark_short,
    benchmark_equal,
    bench_comparison,
);

criterion_main!(benches);
