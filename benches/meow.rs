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
            use crate::{
                __ahash_hash,
                __default_hash
            };
            use fxhash as _fxhash;
            use ahash as _ahash;
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
generate_functions!(ahash, __ahash_hash);
generate_functions!(fasthash, _metro::hash64);
generate_functions!(default_hasher, __default_hash);

#[inline(always)]
fn __ahash_hash(input: &[u8]) -> u64 {
    let mut hasher = ::ahash::AHasher::default();
    hasher.write(input);
    hasher.finish()
}

#[inline]
fn __default_hash(input: &[u8]) -> u64 {
    let mut hasher = ::ahash::AHasher::default();
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
