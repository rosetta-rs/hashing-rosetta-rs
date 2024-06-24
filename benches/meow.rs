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
            paste! {
            #[divan::bench]
            fn short() -> bool {
                let a = $hash_fn(include_bytes!("../article1.txt"));
                let b = $hash_fn(include_bytes!("../article2.txt"));

                // False
                a == b
            }

            #[divan::bench]
            fn long() -> bool {
                let a = $hash_fn(include_bytes!("../article3.txt"));
                let b = $hash_fn(include_bytes!("../article4.txt"));

                // False
                a == b
            }

            #[divan::bench]
            fn equal() -> bool {
                let a = $hash_fn(include_bytes!("../article4.txt"));
                let b = $hash_fn(include_bytes!("../article5.txt"));

                // True
                a == b
            }}
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
generate_functions!(hud_slice_by_8, __hash::<::hud_slice_by_8::crc32::CRC32Hasher>);


fn __hash<H: Hasher + Default>(input: &[u8]) -> u64 {
    let mut hasher = H::default();
    hasher.write(input);
    hasher.finish()
}


#[divan::bench]
fn comparison_short() -> bool {
    let a = include_str!("../article1.txt");
    let b = include_str!("../article2.txt");

    // False
    a == b
}

#[divan::bench]
fn comparison_long() -> bool {
    let a = include_str!("../article3.txt");
    let b = include_str!("../article4.txt");

    // False
    a == b
}

#[divan::bench]
fn comparison_true()-> bool {
    let a = include_str!("../article4.txt");
    let b = include_str!("../article5.txt");

    // True
    a == b
}

fn main() {
    divan::main();
}
