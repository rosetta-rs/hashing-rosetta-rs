use fxhash;
use twox_hash;

use divan;
use paste::paste;

macro_rules! generate_functions {
    ($name: expr, $hash_fn: expr) => {
        paste! {
            #[divan::bench]
            fn [<$name _short>]() -> bool {
                let a = $hash_fn(include_bytes!("../article1.txt"));
                let b = $hash_fn(include_bytes!("../article2.txt"));

                // False
                a == b
            }

            #[divan::bench]
            fn [<$name _long>]() -> bool {
                let a = $hash_fn(include_bytes!("../article3.txt"));
                let b = $hash_fn(include_bytes!("../article4.txt"));

                // False
                a == b
            }

            #[divan::bench]
            fn [<$name _true>]() -> bool {
                let a = $hash_fn(include_bytes!("../article4.txt"));
                let b = $hash_fn(include_bytes!("../article5.txt"));

                // True
                a == b
            }
        }
    };
}

generate_functions!(fxhash, fxhash::hash);
generate_functions!(xxh3, twox_hash::xxh3::hash64);

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
