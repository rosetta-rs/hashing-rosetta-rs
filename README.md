# Rosetta hashing experiment

What's faster, hashing a string and comparing the hash, or simply comparing the string? This Rust benchmark measures just that.
 
Here are the results in a benchmark done in my machine, with an AMD Ryzen 5 2600 CPU.

<!-- Benchmark here -->

```plain
long article/xxh3       time:   [306.54 µs 307.62 µs 308.78 µs]
long article/meowhash   time:   [201.29 µs 202.30 µs 203.34 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
long article/ahash      time:   [211.37 µs 212.29 µs 213.47 µs]
Found 12 outliers among 100 measurements (12.00%)
  7 (7.00%) high mild
  5 (5.00%) high severe
long article/fasthash   time:   [716.67 µs 717.63 µs 718.64 µs]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe
Benchmarking long article/default_hasher: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 7.0s, enable flat sampling, or reduce sample count to 50.
long article/default_hasher
                        time:   [1.3890 ms 1.3896 ms 1.3903 ms]
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe
long article/rustc_hash time:   [357.02 µs 360.66 µs 364.69 µs]
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) high mild
  7 (7.00%) high severe
long article/fnv        time:   [6.9346 ms 6.9387 ms 6.9440 ms]
Found 15 outliers among 100 measurements (15.00%)
  1 (1.00%) high mild
  14 (14.00%) high severe
long article/xxhash_rust_xxh3
                        time:   [320.39 µs 322.56 µs 324.50 µs]
long article/highway    time:   [589.14 µs 589.80 µs 590.49 µs]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
long article/cityhasher time:   [409.54 µs 411.62 µs 414.95 µs]
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  4 (4.00%) high severe
long article/gxhash     time:   [170.80 µs 173.48 µs 175.93 µs]
long article/wyhash     time:   [406.59 µs 408.02 µs 409.37 µs]
Found 11 outliers among 100 measurements (11.00%)
  8 (8.00%) low severe
  3 (3.00%) low mild
long article/blake3     time:   [3.0515 ms 3.0768 ms 3.0999 ms]
Found 17 outliers among 100 measurements (17.00%)
  15 (15.00%) low severe
  2 (2.00%) high mild
long article/hud_slice_by_8
                        time:   [2.6158 ms 2.6217 ms 2.6284 ms]
Found 16 outliers among 100 measurements (16.00%)
  9 (9.00%) high mild
  7 (7.00%) high severe
long article/rapidhash  time:   [286.57 µs 289.37 µs 292.30 µs]
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) low mild
  6 (6.00%) high mild

short article/xxh3      time:   [4.5455 µs 4.5560 µs 4.5684 µs]
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe
short article/meowhash  time:   [3.5200 µs 3.5241 µs 3.5283 µs]
Found 19 outliers among 100 measurements (19.00%)
  1 (1.00%) low severe
  9 (9.00%) low mild
  6 (6.00%) high mild
  3 (3.00%) high severe
short article/ahash     time:   [3.1434 µs 3.1464 µs 3.1494 µs]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  1 (1.00%) high severe
short article/fasthash  time:   [13.350 µs 13.358 µs 13.369 µs]
Found 16 outliers among 100 measurements (16.00%)
  1 (1.00%) low severe
  6 (6.00%) high mild
  9 (9.00%) high severe
short article/default_hasher
                        time:   [27.089 µs 27.144 µs 27.220 µs]
Found 12 outliers among 100 measurements (12.00%)
  1 (1.00%) low severe
  4 (4.00%) high mild
  7 (7.00%) high severe
short article/rustc_hash
                        time:   [6.4883 µs 6.4931 µs 6.4992 µs]
Found 21 outliers among 100 measurements (21.00%)
  2 (2.00%) low severe
  8 (8.00%) low mild
  4 (4.00%) high mild
  7 (7.00%) high severe
short article/fnv       time:   [135.94 µs 136.04 µs 136.18 µs]
Found 11 outliers among 100 measurements (11.00%)
  1 (1.00%) low severe
  5 (5.00%) high mild
  5 (5.00%) high severe
short article/xxhash_rust_xxh3
                        time:   [4.7071 µs 4.7137 µs 4.7205 µs]
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) low mild
  2 (2.00%) high mild
  2 (2.00%) high severe
short article/highway   time:   [11.445 µs 11.467 µs 11.489 µs]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild
short article/cityhasher
                        time:   [7.6349 µs 7.6459 µs 7.6594 µs]
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) high mild
  5 (5.00%) high severe
short article/gxhash    time:   [1.8882 µs 1.8931 µs 1.8982 µs]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high severe
short article/wyhash    time:   [6.5771 µs 6.5922 µs 6.6068 µs]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
short article/blake3    time:   [53.345 µs 53.392 µs 53.450 µs]
Found 16 outliers among 100 measurements (16.00%)
  3 (3.00%) high mild
  13 (13.00%) high severe
short article/hud_slice_by_8
                        time:   [50.933 µs 50.986 µs 51.051 µs]
Found 14 outliers among 100 measurements (14.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  2 (2.00%) high mild
  9 (9.00%) high severe
short article/rapidhash time:   [4.5694 µs 4.5757 µs 4.5829 µs]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild

long article/xxh3 #2    time:   [266.32 µs 266.44 µs 266.57 µs]
Found 11 outliers among 100 measurements (11.00%)
  6 (6.00%) high mild
  5 (5.00%) high severe
long article/meowhash #2
                        time:   [160.78 µs 161.08 µs 161.51 µs]
Found 15 outliers among 100 measurements (15.00%)
  3 (3.00%) low mild
  1 (1.00%) high mild
  11 (11.00%) high severe
long article/ahash #2   time:   [161.36 µs 161.70 µs 162.01 µs]
long article/fasthash #2
                        time:   [651.01 µs 653.54 µs 657.49 µs]
Found 12 outliers among 100 measurements (12.00%)
  2 (2.00%) high mild
  10 (10.00%) high severe
Benchmarking long article/default_hasher #2: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 7.0s, enable flat sampling, or reduce sample count to 50.
long article/default_hasher #2
                        time:   [1.3808 ms 1.3824 ms 1.3845 ms]
Found 8 outliers among 100 measurements (8.00%)
  2 (2.00%) high mild
  6 (6.00%) high severe
long article/rustc_hash #2
                        time:   [328.16 µs 328.55 µs 329.00 µs]
Found 15 outliers among 100 measurements (15.00%)
  3 (3.00%) high mild
  12 (12.00%) high severe
long article/fnv #2     time:   [6.9401 ms 6.9469 ms 6.9551 ms]
Found 20 outliers among 100 measurements (20.00%)
  7 (7.00%) high mild
  13 (13.00%) high severe
long article/xxhash_rust_xxh3 #2
                        time:   [275.03 µs 275.28 µs 275.53 µs]
Found 21 outliers among 100 measurements (21.00%)
  1 (1.00%) low severe
  11 (11.00%) low mild
  6 (6.00%) high mild
  3 (3.00%) high severe
long article/highway #2 time:   [584.52 µs 587.15 µs 589.40 µs]
long article/cityhasher #2
                        time:   [385.73 µs 386.17 µs 386.65 µs]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
long article/gxhash #2  time:   [109.86 µs 109.91 µs 109.97 µs]
Found 16 outliers among 100 measurements (16.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  4 (4.00%) high mild
  10 (10.00%) high severe
long article/wyhash #2  time:   [337.53 µs 339.36 µs 341.39 µs]
long article/blake3 #2  time:   [2.8119 ms 2.8444 ms 2.8766 ms]
long article/hud_slice_by_8 #2
                        time:   [2.5882 ms 2.5908 ms 2.5935 ms]
long article/rapidhash #2
                        time:   [238.33 µs 239.99 µs 241.97 µs]

== comparison/short     time:   [2.0746 ns 2.0758 ns 2.0771 ns]
Found 11 outliers among 100 measurements (11.00%)
  4 (4.00%) low mild
  3 (3.00%) high mild
  4 (4.00%) high severe
== comparison/long      time:   [2.0664 ns 2.0676 ns 2.0688 ns]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
== comparison/true      time:   [69.121 µs 69.209 µs 69.277 µs]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) low severe
  1 (1.00%) high mild

     Running benches/small.rs (target/release/deps/small-d93abed6630a27af)
3 bytes (1000 times)/xxh3
                        time:   [3.3601 µs 3.3686 µs 3.3791 µs]
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe
3 bytes (1000 times)/meowhash
                        time:   [41.836 µs 41.869 µs 41.905 µs]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe
3 bytes (1000 times)/ahash
                        time:   [4.7886 µs 4.8034 µs 4.8164 µs]
3 bytes (1000 times)/fasthash
                        time:   [5.5338 µs 5.5395 µs 5.5455 µs]
Found 10 outliers among 100 measurements (10.00%)
  3 (3.00%) high mild
  7 (7.00%) high severe
3 bytes (1000 times)/default_hasher
                        time:   [8.9491 µs 8.9587 µs 8.9689 µs]
3 bytes (1000 times)/rustc_hash
                        time:   [2.6371 µs 2.6467 µs 2.6584 µs]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
3 bytes (1000 times)/fnv
                        time:   [2.8621 µs 2.8648 µs 2.8675 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe
3 bytes (1000 times)/xxhash_rust_xxh3
                        time:   [17.504 µs 17.608 µs 17.724 µs]
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe
3 bytes (1000 times)/highway
                        time:   [37.590 µs 37.622 µs 37.654 µs]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe
3 bytes (1000 times)/cityhasher
                        time:   [7.3135 µs 7.3485 µs 7.3906 µs]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
3 bytes (1000 times)/gxhash
                        time:   [13.893 µs 13.904 µs 13.919 µs]
Found 21 outliers among 100 measurements (21.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  2 (2.00%) high mild
  17 (17.00%) high severe
3 bytes (1000 times)/wyhash
                        time:   [5.3448 µs 5.3771 µs 5.4129 µs]
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe
3 bytes (1000 times)/blake3
                        time:   [84.932 µs 85.067 µs 85.275 µs]
Found 21 outliers among 100 measurements (21.00%)
  1 (1.00%) low severe
  8 (8.00%) low mild
  5 (5.00%) high mild
  7 (7.00%) high severe
3 bytes (1000 times)/hud_slice_by_8
                        time:   [6.7793 µs 6.8111 µs 6.8522 µs]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) low mild
  3 (3.00%) high severe
3 bytes (1000 times)/rapidhash
                        time:   [3.0560 µs 3.0647 µs 3.0731 µs]
Found 8 outliers among 100 measurements (8.00%)
  6 (6.00%) high mild
  2 (2.00%) high severe

10 bytes (1000 times)/xxh3
                        time:   [12.492 µs 12.504 µs 12.520 µs]
Found 11 outliers among 100 measurements (11.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  2 (2.00%) high mild
  6 (6.00%) high severe
10 bytes (1000 times)/meowhash
                        time:   [42.879 µs 42.983 µs 43.101 µs]
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) low mild
  3 (3.00%) high mild
  1 (1.00%) high severe
10 bytes (1000 times)/ahash
                        time:   [13.603 µs 13.623 µs 13.644 µs]
10 bytes (1000 times)/fasthash
                        time:   [16.216 µs 16.228 µs 16.243 µs]
Found 18 outliers among 100 measurements (18.00%)
  2 (2.00%) low mild
  2 (2.00%) high mild
  14 (14.00%) high severe
10 bytes (1000 times)/default_hasher
                        time:   [16.328 µs 16.346 µs 16.368 µs]
Found 13 outliers among 100 measurements (13.00%)
  1 (1.00%) high mild
  12 (12.00%) high severe
10 bytes (1000 times)/rustc_hash
                        time:   [11.598 µs 11.604 µs 11.612 µs]
Found 10 outliers among 100 measurements (10.00%)
  2 (2.00%) low mild
  1 (1.00%) high mild
  7 (7.00%) high severe
10 bytes (1000 times)/fnv
                        time:   [6.0300 µs 6.0722 µs 6.1166 µs]
Found 20 outliers among 100 measurements (20.00%)
  2 (2.00%) low mild
  7 (7.00%) high mild
  11 (11.00%) high severe
10 bytes (1000 times)/xxhash_rust_xxh3
                        time:   [24.770 µs 24.809 µs 24.852 µs]
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe
10 bytes (1000 times)/highway
                        time:   [37.955 µs 38.007 µs 38.058 µs]
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low mild
  5 (5.00%) high mild
  1 (1.00%) high severe
10 bytes (1000 times)/cityhasher
                        time:   [15.486 µs 15.503 µs 15.524 µs]
Found 19 outliers among 100 measurements (19.00%)
  3 (3.00%) low mild
  2 (2.00%) high mild
  14 (14.00%) high severe
10 bytes (1000 times)/gxhash
                        time:   [13.667 µs 13.687 µs 13.713 µs]
Found 20 outliers among 100 measurements (20.00%)
  2 (2.00%) low severe
  5 (5.00%) low mild
  2 (2.00%) high mild
  11 (11.00%) high severe
10 bytes (1000 times)/wyhash
                        time:   [12.120 µs 12.133 µs 12.150 µs]
Found 13 outliers among 100 measurements (13.00%)
  3 (3.00%) high mild
  10 (10.00%) high severe
10 bytes (1000 times)/blake3
                        time:   [83.617 µs 83.705 µs 83.808 µs]
Found 7 outliers among 100 measurements (7.00%)
  2 (2.00%) low mild
  3 (3.00%) high mild
  2 (2.00%) high severe
10 bytes (1000 times)/hud_slice_by_8
                        time:   [14.812 µs 14.832 µs 14.854 µs]
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe
10 bytes (1000 times)/rapidhash
                        time:   [12.496 µs 12.508 µs 12.525 µs]
Found 12 outliers among 100 measurements (12.00%)
  1 (1.00%) low severe
  2 (2.00%) high mild
  9 (9.00%) high severe

100 bytes (1000 times)/xxh3
                        time:   [16.300 µs 16.321 µs 16.347 µs]
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe
100 bytes (1000 times)/meowhash
                        time:   [51.016 µs 51.295 µs 51.579 µs]
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe
100 bytes (1000 times)/ahash
                        time:   [22.732 µs 22.741 µs 22.751 µs]
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) low severe
  3 (3.00%) low mild
  2 (2.00%) high mild
  1 (1.00%) high severe
100 bytes (1000 times)/fasthash
                        time:   [28.327 µs 28.355 µs 28.393 µs]
Found 16 outliers among 100 measurements (16.00%)
  1 (1.00%) high mild
  15 (15.00%) high severe
100 bytes (1000 times)/default_hasher
                        time:   [29.921 µs 29.957 µs 30.004 µs]
Found 21 outliers among 100 measurements (21.00%)
  2 (2.00%) low mild
  9 (9.00%) high mild
  10 (10.00%) high severe
100 bytes (1000 times)/rustc_hash
                        time:   [17.366 µs 17.380 µs 17.398 µs]
Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) high mild
  7 (7.00%) high severe
100 bytes (1000 times)/fnv
                        time:   [99.967 µs 100.09 µs 100.24 µs]
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe
100 bytes (1000 times)/xxhash_rust_xxh3
                        time:   [34.523 µs 34.609 µs 34.706 µs]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  1 (1.00%) high severe
100 bytes (1000 times)/highway
                        time:   [44.650 µs 44.717 µs 44.790 µs]
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe
100 bytes (1000 times)/cityhasher
                        time:   [29.574 µs 29.600 µs 29.629 µs]
Found 15 outliers among 100 measurements (15.00%)
  2 (2.00%) low mild
  8 (8.00%) high mild
  5 (5.00%) high severe
100 bytes (1000 times)/gxhash
                        time:   [22.603 µs 22.613 µs 22.625 µs]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe
100 bytes (1000 times)/wyhash
                        time:   [18.231 µs 18.248 µs 18.265 µs]
100 bytes (1000 times)/blake3
                        time:   [153.26 µs 153.32 µs 153.38 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) low mild
100 bytes (1000 times)/hud_slice_by_8
                        time:   [46.995 µs 47.051 µs 47.110 µs]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe
100 bytes (1000 times)/rapidhash
                        time:   [17.783 µs 17.802 µs 17.829 µs]
Found 21 outliers among 100 measurements (21.00%)
  1 (1.00%) low severe
  2 (2.00%) low mild
  1 (1.00%) high mild
  17 (17.00%) high severe
```

As you can see, the default `stringA == stringB` is about 10^6 times faster than the fastest hashing algorithm. So yeah, hashing is slower than comparing (at least in Rust). This was tested on the `article*.txt` files you see on the file tree.

To see the full Criterion report [check here](https://github.com/rosetta-rs/hashing-rosetta-rs/blob/master/criterion.tar.gz)

## Running the benchmarks yourself

(Git and Cargo must be installed)

Just clone this repo with `git clone https://github.com/rosetta-rs/hashing-rosetta-rs` or download the ZIP with the green button, and run `RUSTFLAGS="-C target-cpu=native"cargo bench`.
