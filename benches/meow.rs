use fxhash;
use divan;

#[divan::bench]
fn hashing_short() -> bool {
    let a = fxhash::hash(include_str!("../article.txt"));
    let b = fxhash::hash(include_str!("../article2.txt"));

    a == b
}

#[divan::bench]
fn comparison_short() -> bool {
    let a = include_str!("../article.txt");
    let b = include_str!("../article2.txt");

    a == b
}

#[divan::bench]
fn hashing_long() -> bool {
    let a = fxhash::hash(include_str!("../article3.txt"));
    let b = fxhash::hash(include_str!("../article4.txt"));

    a == b
}

#[divan::bench]
fn comparison_long() -> bool {
    let a = include_str!("../article3.txt");
    let b = include_str!("../article4.txt");

    a == b
}

fn main() {
    divan::main();
}
