# Rosetta hashing experiment

What's faster, hashing a string and comparing the hash, or simply comparing the string? This Rust benchmark measures just that.
 
Here are the results in a benchmark done in my machine, with an AMD Ryzen 5 2600 CPU.

<!-- Benchmark here -->

```plain
     Running benches/hashing.rs (target/release/deps/hashing-398dda6a7aeee9d1)
long article/xxh3       time:   [360.11 µs 365.86 µs 371.83 µs]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
long article/meowhash   time:   [265.79 µs 270.88 µs 276.12 µs]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
long article/ahash      time:   [270.11 µs 273.73 µs 277.40 µs]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
long article/fasthash   time:   [814.09 µs 820.96 µs 827.82 µs]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
Benchmarking long article/default_hasher: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 7.6s, enable flat sampling, or reduce sample count to 50.
long article/default_hasher
                        time:   [1.4628 ms 1.4692 ms 1.4756 ms]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe
long article/rustc_hash time:   [459.34 µs 466.83 µs 474.46 µs]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low mild
  3 (3.00%) high mild
long article/fnv        time:   [7.0690 ms 7.0911 ms 7.1152 ms]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe
long article/xxhash_rust_xxh3
                        time:   [395.06 µs 401.30 µs 408.05 µs]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  1 (1.00%) high severe
long article/highway    time:   [661.28 µs 667.94 µs 674.70 µs]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
long article/cityhasher time:   [493.48 µs 498.47 µs 503.84 µs]
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  2 (2.00%) high severe
long article/gxhash     time:   [223.53 µs 227.76 µs 231.89 µs]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
long article/wyhash     time:   [450.00 µs 455.09 µs 459.92 µs]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
long article/blake3     time:   [3.1147 ms 3.1561 ms 3.1993 ms]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
long article/hud_slice_by_8
                        time:   [2.7176 ms 2.7282 ms 2.7391 ms]
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild

short article/xxh3      time:   [5.0451 µs 5.1039 µs 5.1657 µs]
Found 8 outliers among 100 measurements (8.00%)
  6 (6.00%) high mild
  2 (2.00%) high severe
short article/meowhash  time:   [3.6227 µs 3.6376 µs 3.6538 µs]
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe
short article/ahash     time:   [3.3402 µs 3.3721 µs 3.4077 µs]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
short article/fasthash  time:   [14.076 µs 14.145 µs 14.222 µs]
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe
short article/default_hasher
                        time:   [28.108 µs 28.360 µs 28.654 µs]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe
short article/rustc_hash
                        time:   [6.8942 µs 6.9465 µs 7.0034 µs]
short article/fnv       time:   [137.33 µs 137.52 µs 137.71 µs]
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe
short article/xxhash_rust_xxh3
                        time:   [5.2465 µs 5.3275 µs 5.4104 µs]
Found 7 outliers among 100 measurements (7.00%)
  7 (7.00%) high mild
short article/highway   time:   [11.465 µs 11.531 µs 11.606 µs]
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe
short article/cityhasher
                        time:   [7.9841 µs 8.0283 µs 8.0729 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe
short article/gxhash    time:   [1.9131 µs 1.9261 µs 1.9391 µs]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
short article/wyhash    time:   [7.0290 µs 7.0850 µs 7.1453 µs]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe
short article/blake3    time:   [58.354 µs 58.910 µs 59.452 µs]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe
short article/hud_slice_by_8
                        time:   [53.056 µs 53.297 µs 53.561 µs]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe

long article/xxh3 #2    time:   [284.13 µs 286.35 µs 288.71 µs]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
long article/meowhash #2
                        time:   [197.03 µs 198.45 µs 199.86 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
long article/ahash #2   time:   [173.64 µs 175.24 µs 176.88 µs]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
long article/fasthash #2
                        time:   [749.50 µs 754.16 µs 759.43 µs]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
Benchmarking long article/default_hasher #2: Warming up for 3.0000 s
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 7.4s, enable flat sampling, or reduce sample count to 50.
long article/default_hasher #2
                        time:   [1.4833 ms 1.4971 ms 1.5120 ms]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
long article/rustc_hash #2
                        time:   [385.13 µs 387.60 µs 390.31 µs]
Found 7 outliers among 100 measurements (7.00%)
  7 (7.00%) high mild
long article/fnv #2     time:   [7.0786 ms 7.1247 ms 7.1811 ms]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) high mild
  5 (5.00%) high severe
long article/xxhash_rust_xxh3 #2
                        time:   [297.87 µs 301.27 µs 304.91 µs]
long article/highway #2 time:   [594.74 µs 599.08 µs 603.88 µs]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
long article/cityhasher #2
                        time:   [416.98 µs 420.40 µs 424.08 µs]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe
long article/gxhash #2  time:   [122.49 µs 122.98 µs 123.52 µs]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe
long article/wyhash #2  time:   [396.98 µs 401.12 µs 405.20 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
long article/blake3 #2  time:   [3.0959 ms 3.1280 ms 3.1611 ms]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
long article/hud_slice_by_8 #2
                        time:   [2.6893 ms 2.7001 ms 2.7112 ms]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild

== comparison/short     time:   [2.1493 ns 2.1593 ns 2.1701 ns]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe
== comparison/long      time:   [2.1194 ns 2.1282 ns 2.1369 ns]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
== comparison/true      time:   [69.592 µs 70.251 µs 70.840 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

     Running benches/small.rs (target/release/deps/small-723a008c8061be06)
3 bytes (1000 times)/xxh3
                        time:   [4.1116 µs 4.1811 µs 4.2525 µs]
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) low mild
3 bytes (1000 times)/meowhash
                        time:   [47.834 µs 48.917 µs 50.001 µs]
3 bytes (1000 times)/ahash
                        time:   [5.3466 µs 5.4952 µs 5.6381 µs]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe
3 bytes (1000 times)/fasthash
                        time:   [6.1808 µs 6.2895 µs 6.4057 µs]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
3 bytes (1000 times)/default_hasher
                        time:   [9.9716 µs 10.065 µs 10.161 µs]
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe
3 bytes (1000 times)/rustc_hash
                        time:   [3.0213 µs 3.0850 µs 3.1601 µs]
3 bytes (1000 times)/fnv
                        time:   [3.1022 µs 3.1393 µs 3.1793 µs]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
3 bytes (1000 times)/xxhash_rust_xxh3
                        time:   [19.356 µs 19.598 µs 19.846 µs]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
3 bytes (1000 times)/highway
                        time:   [40.717 µs 41.101 µs 41.495 µs]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
3 bytes (1000 times)/cityhasher
                        time:   [8.3796 µs 8.5445 µs 8.7406 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
3 bytes (1000 times)/gxhash
                        time:   [13.222 µs 13.251 µs 13.280 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
3 bytes (1000 times)/wyhash
                        time:   [6.6518 µs 6.7212 µs 6.7847 µs]
3 bytes (1000 times)/blake3
                        time:   [88.430 µs 89.040 µs 89.642 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
3 bytes (1000 times)/hud_slice_by_8
                        time:   [7.9541 µs 8.1554 µs 8.3395 µs]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

10 bytes (1000 times)/xxh3
                        time:   [13.361 µs 13.407 µs 13.457 µs]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild
10 bytes (1000 times)/meowhash
                        time:   [47.581 µs 48.307 µs 49.039 µs]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild
10 bytes (1000 times)/ahash
                        time:   [14.337 µs 14.411 µs 14.493 µs]
Found 10 outliers among 100 measurements (10.00%)
  7 (7.00%) high mild
  3 (3.00%) high severe
10 bytes (1000 times)/fasthash
                        time:   [16.461 µs 16.502 µs 16.544 µs]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild
10 bytes (1000 times)/default_hasher
                        time:   [16.948 µs 17.017 µs 17.091 µs]
Found 11 outliers among 100 measurements (11.00%)
  8 (8.00%) high mild
  3 (3.00%) high severe
10 bytes (1000 times)/rustc_hash
                        time:   [12.530 µs 12.548 µs 12.569 µs]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe
10 bytes (1000 times)/fnv
                        time:   [7.1865 µs 7.3080 µs 7.4252 µs]
Found 3 outliers among 100 measurements (3.00%)
  3 (3.00%) high mild
10 bytes (1000 times)/xxhash_rust_xxh3
                        time:   [25.652 µs 25.755 µs 25.870 µs]
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe
10 bytes (1000 times)/highway
                        time:   [40.261 µs 40.754 µs 41.320 µs]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
10 bytes (1000 times)/cityhasher
                        time:   [16.024 µs 16.057 µs 16.092 µs]
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe
10 bytes (1000 times)/gxhash
                        time:   [13.187 µs 13.217 µs 13.247 µs]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
10 bytes (1000 times)/wyhash
                        time:   [12.472 µs 12.518 µs 12.571 µs]
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe
10 bytes (1000 times)/blake3
                        time:   [86.626 µs 87.082 µs 87.573 µs]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
10 bytes (1000 times)/hud_slice_by_8
                        time:   [14.992 µs 15.037 µs 15.086 µs]
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe

100 bytes (1000 times)/xxh3
                        time:   [17.051 µs 17.115 µs 17.185 µs]
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe
100 bytes (1000 times)/meowhash
                        time:   [57.203 µs 58.119 µs 59.038 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
100 bytes (1000 times)/ahash
                        time:   [23.247 µs 23.345 µs 23.437 µs]
Found 8 outliers among 100 measurements (8.00%)
  7 (7.00%) high mild
  1 (1.00%) high severe
100 bytes (1000 times)/fasthash
                        time:   [29.579 µs 29.709 µs 29.848 µs]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
100 bytes (1000 times)/default_hasher
                        time:   [30.265 µs 30.521 µs 30.776 µs]
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe
100 bytes (1000 times)/rustc_hash
                        time:   [17.957 µs 18.010 µs 18.073 µs]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
100 bytes (1000 times)/fnv
                        time:   [101.52 µs 101.90 µs 102.42 µs]
Found 9 outliers among 100 measurements (9.00%)
  5 (5.00%) high mild
  4 (4.00%) high severe
100 bytes (1000 times)/xxhash_rust_xxh3
                        time:   [36.268 µs 36.539 µs 36.826 µs]
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe
100 bytes (1000 times)/highway
                        time:   [50.776 µs 51.446 µs 52.177 µs]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild
100 bytes (1000 times)/cityhasher
                        time:   [30.535 µs 30.638 µs 30.746 µs]
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild
100 bytes (1000 times)/gxhash
                        time:   [22.754 µs 22.832 µs 22.922 µs]
Found 8 outliers among 100 measurements (8.00%)
  6 (6.00%) high mild
  2 (2.00%) high severe
100 bytes (1000 times)/wyhash
                        time:   [18.199 µs 18.278 µs 18.357 µs]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
100 bytes (1000 times)/blake3
                        time:   [157.20 µs 158.13 µs 159.18 µs]
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe
100 bytes (1000 times)/hud_slice_by_8
                        time:   [50.536 µs 51.359 µs 52.232 µs]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
```

As you can see, the default `stringA == stringB` is about 10^6 times faster than the fastest hashing algorithm. So yeah, hashing is slower than comparing (at least in Rust). This was tested on the `article*.txt` files you see on the file tree.

To see the full Criterion report [check here](https://github.com/rosetta-rs/hashing-rosetta-rs/blob/master/criterion.tar.gz)

## Running the benchmarks yourself

(Git and Cargo must be installed)

Just clone this repo with `git clone https://github.com/rosetta-rs/hashing-rosetta-rs` or download the ZIP with the green button, and run `RUSTFLAGS="-C target-cpu=native"cargo bench`.
