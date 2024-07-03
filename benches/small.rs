use std::hash::Hasher;

use divan;

macro_rules! generate_functions {
    ($name: ident, $hash_fn: expr) => {
        #[divan::bench_group]
        #[allow(unused)]
        mod $name {
            use paste::paste;
            use meowhash as _meowhash;
            use fasthash::metro as _metro;
            use crate::__hash;
            use fxhash as _fxhash;
            use ahash as _ahash;
            use fnv as _fnv;
            use std::hash::DefaultHasher;
            use std::hint::black_box;
            #[divan::bench]
            fn _3_bytes() {
                $hash_fn(black_box(&[0xd4, 0x23, 0x01]));
            }
            #[divan::bench]
            fn _10_bytes() {
                $hash_fn(black_box(&[0xd4, 0x23, 0x01, 0x2a, 0x40, 0xb7, 0x42, 0x92, 0xe5, 0x35]));
            }
            #[divan::bench]
            fn _30_bytes() {
                $hash_fn(black_box(&[86, 222, 109, 153, 157, 251, 151, 200, 214, 152, 242, 196, 110, 205, 14, 251,
                    161, 176, 133, 185, 189, 241, 232, 163, 113, 193,  29, 163,  59, 196, ]));
            }
            #[divan::bench]
            fn _100_bytes() {
                $hash_fn(black_box(&[228, 211, 236, 162, 172, 193,  22, 203, 162, 253, 169,  29,  50, 148,  19, 246, 
                    120,  13, 137, 153,  47,  33,  31, 217,  18,  70,  66,  32, 249,  86, 189, 117, 
                    134,  37, 185,  72, 130,  52, 170,  92, 231, 185, 110, 224,  60, 123, 234, 218, 
                     23, 119, 207, 156,  32,  41, 143,  35, 127, 202, 126, 110, 148, 149, 247, 188, 
                     71,   2, 169, 206,  38, 147,  23, 249,  88, 230, 228, 236, 164,  32,  87,  95, 
                    154, 204, 225, 154,  94,  53, 182, 109,  16, 207,  20,  54,  60, 109,  98, 139, 
                    237,  41, 131, 220, ]));
            }

            #[divan::bench]
            fn _3_bytes_x1000() {
                for i in 0..1000 {
                black_box($hash_fn(black_box(&[0xd4, 0x23, (i % 255) as u8])));
                }
            }
            #[divan::bench]
            fn _10_bytes_x1000() {
                for i in 0..1000 {
                black_box($hash_fn(black_box(&[0xd4, 0x23, (i % 255) as u8, 0x2a, 0x40, 0xb7, 0x42, 0x92, 0xe5, 0x35])));
                }
            }
            #[divan::bench]
            fn _30_bytes_x1000() {
                for i in 0..1000 {
                black_box($hash_fn(black_box(&[86, 222, 109, 153, 157, 251, 151, 200, 214, 152, 242, 196, 110, 205, 14, 251,
                    161, 176, 133, 185, (i % 255) as u8, 189, 241, 232, 163, 113, 193,  29, 163,  59, 196])));
                }
            }
            #[divan::bench]
            fn _100_bytes_x1000() {
                for i in 0..1000 {
                black_box($hash_fn(black_box(&[228, 211, 236, 162, 172, 193,  22, 203, 162, 253, 169,  29,  50, 148,  19, 246, 
                    120,  13, 137, 153,  47,  33,  31, 217,  18,  70,  66,  32, 249,  86, 189, 117, 
                    134,  37, 185,  72, 130,  52, 170,  92, 231, 185, 110, 224,  60, 123, 234, 218, 
                     23, 119, 207, 156,  32,  41, 143,  35, 127, 202, 126, 110, 148, 149, 247, 188, 
                     71,   2, 169, (i % 255) as u8, 206,  38, 147,  23, 249,  88, 230, 228, 236, 164,  32,  87,  95, 
                    154, 204, 225, 154,  94,  53, 182, 109,  16, 207,  20,  54,  60, 109,  98, 139, 
                    237,  41, 131, 220, ])));
                }
            }
        }
    };
}

generate_functions!(fxhash, _fxhash::hash);
generate_functions!(xxh3, twox_hash::xxh3::hash64);
generate_functions!(meowhash, _meowhash::MeowHasher::hash);
generate_functions!(ahash, __hash::<::ahash::AHasher>);
generate_functions!(fasthash, _metro::hash64);
generate_functions!(default_hasher, __hash::<DefaultHasher>);
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

fn __hash<H: Hasher + Default>(input: &[u8]) -> u64 {
    let mut hasher = H::default();
    hasher.write(input);
    hasher.finish()
}

fn main() {
    divan::main();
}

