use std::hash::Hasher;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_3_bytes(c: &mut Criterion) {
    let mut group = c.benchmark_group("3 bytes (1000 times)");

    macro_rules! generate_functions {
        ($name: ident, $hash_fn: expr) => {{
            group.bench_function(stringify!($name), |b| b.iter(|| bench()));
            fn bench() {
                for i in 0..1000 {
                    black_box($hash_fn(black_box(&[0xd4, 0x23, (i % 255) as u8])));
                }
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

fn benchmark_10_bytes(c: &mut Criterion) {
    let mut group = c.benchmark_group("10 bytes (1000 times)");

    macro_rules! generate_functions {
        ($name: ident, $hash_fn: expr) => {{
            group.bench_function(stringify!($name), |b| b.iter(|| bench()));
            fn bench() {
                for i in 0..1000 {
                    black_box($hash_fn(black_box(&[
                        0xd4,
                        0x23,
                        (i % 255) as u8,
                        0x2a,
                        0x40,
                        0xb7,
                        0x42,
                        0x92,
                        0xe5,
                        0x35,
                    ])));
                }
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

fn benchmark_100_bytes(c: &mut Criterion) {
    let mut group = c.benchmark_group("100 bytes (1000 times)");

    macro_rules! generate_functions {
        ($name: ident, $hash_fn: expr) => {{
            group.bench_function(stringify!($name), |b| b.iter(|| bench()));
            fn bench() {
                for i in 0..1000 {
                    black_box($hash_fn(black_box(&[
                        228,
                        211,
                        236,
                        162,
                        172,
                        193,
                        22,
                        203,
                        162,
                        253,
                        169,
                        29,
                        50,
                        148,
                        19,
                        246,
                        120,
                        13,
                        137,
                        153,
                        47,
                        33,
                        31,
                        217,
                        18,
                        70,
                        66,
                        32,
                        249,
                        86,
                        189,
                        117,
                        134,
                        37,
                        185,
                        72,
                        130,
                        52,
                        170,
                        92,
                        231,
                        185,
                        110,
                        224,
                        60,
                        123,
                        234,
                        218,
                        23,
                        119,
                        207,
                        156,
                        32,
                        41,
                        143,
                        35,
                        127,
                        202,
                        126,
                        110,
                        148,
                        149,
                        247,
                        188,
                        71,
                        2,
                        169,
                        (i % 255) as u8,
                        206,
                        38,
                        147,
                        23,
                        249,
                        88,
                        230,
                        228,
                        236,
                        164,
                        32,
                        87,
                        95,
                        154,
                        204,
                        225,
                        154,
                        94,
                        53,
                        182,
                        109,
                        16,
                        207,
                        20,
                        54,
                        60,
                        109,
                        98,
                        139,
                        237,
                        41,
                        131,
                        220,
                    ])));
                }
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

criterion_group!(
    benches,
    benchmark_3_bytes,
    benchmark_10_bytes,
    benchmark_100_bytes,
);

criterion_main!(benches);
