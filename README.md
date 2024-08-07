# String comparison experiment

What's faster, hashing a string and comparing the hash, or simply comparing the string? This Rust benchmark measures just that.
 
Here are the results in a benchmark done in my machine:

<!-- Benchmark here -->

```plain
Timer precision: 20 ns
hashing              fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ comparison_long   2.696 ns      │ 2.707 ns      │ 2.697 ns      │ 2.7 ns        │ 100     │ 102400
├─ comparison_short  2.325 ns      │ 2.336 ns      │ 2.335 ns      │ 2.333 ns      │ 100     │ 102400
├─ comparison_true   70.69 µs      │ 511.2 µs      │ 75.95 µs      │ 98.17 µs      │ 100     │ 100
├─ ahash                           │               │               │               │         │
│  ├─ equal          498.7 µs      │ 844.2 µs      │ 507 µs        │ 523.6 µs      │ 100     │ 100
│  ├─ long           497.9 µs      │ 907.1 µs      │ 523.3 µs      │ 546 µs        │ 100     │ 100
│  ╰─ short          10.11 µs      │ 27.97 µs      │ 11.13 µs      │ 11.61 µs      │ 100     │ 100
├─ blake3                          │               │               │               │         │
│  ├─ equal          2.67 ms       │ 4.952 ms      │ 2.995 ms      │ 3.005 ms      │ 100     │ 100
│  ├─ long           2.762 ms      │ 5.274 ms      │ 2.841 ms      │ 2.988 ms      │ 100     │ 100
│  ╰─ short          54.63 µs      │ 90.18 µs      │ 65.7 µs       │ 66.54 µs      │ 100     │ 100
├─ cityhasher                      │               │               │               │         │
│  ├─ equal          414.3 µs      │ 687.4 µs      │ 425.6 µs      │ 427.8 µs      │ 100     │ 100
│  ├─ long           435.6 µs      │ 956.8 µs      │ 451.5 µs      │ 493.1 µs      │ 100     │ 100
│  ╰─ short          8.425 µs      │ 13.92 µs      │ 10.12 µs      │ 11.25 µs      │ 100     │ 100
├─ default_hasher                  │               │               │               │         │
│  ├─ equal          1.379 ms      │ 2.069 ms      │ 1.388 ms      │ 1.418 ms      │ 100     │ 100
│  ├─ long           1.394 ms      │ 2.093 ms      │ 1.416 ms      │ 1.456 ms      │ 100     │ 100
│  ╰─ short          27.09 µs      │ 41.58 µs      │ 27.17 µs      │ 30.08 µs      │ 100     │ 100
├─ fasthash                        │               │               │               │         │
│  ├─ equal          777.3 µs      │ 1.244 ms      │ 794.6 µs      │ 831.4 µs      │ 100     │ 100
│  ├─ long           781.1 µs      │ 1.38 ms       │ 824.7 µs      │ 854.1 µs      │ 100     │ 100
│  ╰─ short          14.94 µs      │ 17.48 µs      │ 14.95 µs      │ 15.04 µs      │ 100     │ 100
├─ fnv                             │               │               │               │         │
│  ├─ equal          7.105 ms      │ 7.697 ms      │ 7.147 ms      │ 7.191 ms      │ 100     │ 100
│  ├─ long           7.109 ms      │ 7.461 ms      │ 7.134 ms      │ 7.161 ms      │ 100     │ 100
│  ╰─ short          139.2 µs      │ 153 µs        │ 139.2 µs      │ 139.6 µs      │ 100     │ 100
├─ fxhash                          │               │               │               │         │
│  ├─ equal          1.109 ms      │ 1.279 ms      │ 1.114 ms      │ 1.12 ms       │ 100     │ 100
│  ├─ long           1.114 ms      │ 1.806 ms      │ 1.141 ms      │ 1.158 ms      │ 100     │ 100
│  ╰─ short          22.05 µs      │ 23.47 µs      │ 22.06 µs      │ 22.07 µs      │ 100     │ 100
├─ gxhash                          │               │               │               │         │
│  ├─ equal          1.225 ms      │ 1.596 ms      │ 1.273 ms      │ 1.291 ms      │ 100     │ 100
│  ├─ long           913.5 µs      │ 1.734 ms      │ 951.9 µs      │ 1.076 ms      │ 100     │ 100
│  ╰─ short          17.72 µs      │ 32.16 µs      │ 17.78 µs      │ 17.94 µs      │ 100     │ 100
├─ highway                         │               │               │               │         │
│  ├─ equal          590.5 µs      │ 814.9 µs      │ 605.6 µs      │ 613.4 µs      │ 100     │ 100
│  ├─ long           596.4 µs      │ 1.072 ms      │ 628.2 µs      │ 656 µs        │ 100     │ 100
│  ╰─ short          11.07 µs      │ 14.98 µs      │ 12.04 µs      │ 12.24 µs      │ 100     │ 100
├─ hud_slice_by_8                  │               │               │               │         │
│  ├─ equal          2.719 ms      │ 4.669 ms      │ 2.803 ms      │ 2.861 ms      │ 100     │ 100
│  ├─ long           2.687 ms      │ 3.386 ms      │ 2.723 ms      │ 2.75 ms       │ 100     │ 100
│  ╰─ short          52.65 µs      │ 63.25 µs      │ 52.74 µs      │ 53.07 µs      │ 100     │ 100
├─ meowhash                        │               │               │               │         │
│  ├─ equal          1.086 ms      │ 1.623 ms      │ 1.358 ms      │ 1.366 ms      │ 100     │ 100
│  ├─ long           1.334 ms      │ 1.676 ms      │ 1.355 ms      │ 1.363 ms      │ 100     │ 100
│  ╰─ short          26.73 µs      │ 39.76 µs      │ 26.9 µs       │ 27.35 µs      │ 100     │ 100
├─ rustc_hash                      │               │               │               │         │
│  ├─ equal          352.7 µs      │ 582.5 µs      │ 360 µs        │ 369.4 µs      │ 100     │ 100
│  ├─ long           370.2 µs      │ 647.1 µs      │ 382.3 µs      │ 400.3 µs      │ 100     │ 100
│  ╰─ short          6.802 µs      │ 12.17 µs      │ 7.003 µs      │ 7.093 µs      │ 100     │ 100
├─ wyhash                          │               │               │               │         │
│  ├─ equal          491.6 µs      │ 619.8 µs      │ 495 µs        │ 503.6 µs      │ 100     │ 100
│  ├─ long           401.3 µs      │ 784.1 µs      │ 510.4 µs      │ 526.3 µs      │ 100     │ 100
│  ╰─ short          9.497 µs      │ 14.74 µs      │ 9.713 µs      │ 9.779 µs      │ 100     │ 100
├─ xxh3                            │               │               │               │         │
│  ├─ equal          415.9 µs      │ 819.7 µs      │ 424.1 µs      │ 451.4 µs      │ 100     │ 100
│  ├─ long           436.1 µs      │ 839.7 µs      │ 455 µs        │ 471.7 µs      │ 100     │ 100
│  ╰─ short          8.175 µs      │ 27.99 µs      │ 8.215 µs      │ 8.569 µs      │ 100     │ 100
╰─ xxhash_rust_xxh3                │               │               │               │         │
   ├─ equal          367.9 µs      │ 459.5 µs      │ 379.1 µs      │ 384.1 µs      │ 100     │ 100
   ├─ long           390.4 µs      │ 770.4 µs      │ 422 µs        │ 432 µs        │ 100     │ 100
   ╰─ short          7.293 µs      │ 16.46 µs      │ 7.664 µs      │ 7.813 µs      │ 100     │ 100

Timer precision: 20 ns
small                   fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ ahash                              │               │               │               │         │
│  ├─ _100_bytes        12.41 ns      │ 12.53 ns      │ 12.49 ns      │ 12.47 ns      │ 100     │ 25600
│  ├─ _100_bytes_x1000  19.59 µs      │ 28.9 µs       │ 19.59 µs      │ 19.7 µs       │ 100     │ 100
│  ├─ _10_bytes         5.037 ns      │ 5.078 ns      │ 5.057 ns      │ 5.057 ns      │ 100     │ 51200
│  ├─ _10_bytes_x1000   10.56 µs      │ 12.85 µs      │ 10.56 µs      │ 10.59 µs      │ 100     │ 100
│  ├─ _30_bytes         5.801 ns      │ 5.84 ns       │ 5.82 ns       │ 5.825 ns      │ 100     │ 51200
│  ├─ _30_bytes_x1000   12.15 µs      │ 12.16 µs      │ 12.15 µs      │ 12.15 µs      │ 100     │ 100
│  ├─ _3_bytes          4.781 ns      │ 15.39 ns      │ 4.803 ns      │ 4.901 ns      │ 100     │ 51200
│  ╰─ _3_bytes_x1000    4.698 µs      │ 4.778 µs      │ 4.718 µs      │ 4.715 µs      │ 100     │ 100
├─ blake3                             │               │               │               │         │
│  ├─ _100_bytes        169.7 ns      │ 2.063 µs      │ 170.7 ns      │ 193.6 ns      │ 100     │ 100
│  ├─ _100_bytes_x1000  148 µs        │ 155.8 µs      │ 148.4 µs      │ 148.4 µs      │ 100     │ 100
│  ├─ _10_bytes         78.29 ns      │ 78.64 ns      │ 78.32 ns      │ 78.37 ns      │ 100     │ 3200
│  ├─ _10_bytes_x1000   81.03 µs      │ 91.82 µs      │ 81.03 µs      │ 81.52 µs      │ 100     │ 100
│  ├─ _30_bytes         84.26 ns      │ 243.6 ns      │ 84.29 ns      │ 86.89 ns      │ 100     │ 3200
│  ├─ _30_bytes_x1000   81.23 µs      │ 86.67 µs      │ 81.49 µs      │ 81.63 µs      │ 100     │ 100
│  ├─ _3_bytes          79.23 ns      │ 79.57 ns      │ 79.26 ns      │ 79.36 ns      │ 100     │ 3200
│  ╰─ _3_bytes_x1000    81.24 µs      │ 136.9 µs      │ 81.24 µs      │ 82.17 µs      │ 100     │ 100
├─ cityhasher                         │               │               │               │         │
│  ├─ _100_bytes        20.24 ns      │ 20.55 ns      │ 20.47 ns      │ 20.48 ns      │ 100     │ 12800
│  ├─ _100_bytes_x1000  28.22 µs      │ 36.7 µs       │ 28.26 µs      │ 28.69 µs      │ 100     │ 100
│  ├─ _10_bytes         6.543 ns      │ 11.71 ns      │ 6.582 ns      │ 6.82 ns       │ 100     │ 25600
│  ├─ _10_bytes_x1000   14.45 µs      │ 15.05 µs      │ 14.98 µs      │ 14.97 µs      │ 100     │ 100
│  ├─ _30_bytes         6.955 ns      │ 6.994 ns      │ 6.975 ns      │ 6.977 ns      │ 100     │ 51200
│  ├─ _30_bytes_x1000   14.72 µs      │ 21.43 µs      │ 15.25 µs      │ 15.29 µs      │ 100     │ 100
│  ├─ _3_bytes          6.192 ns      │ 6.252 ns      │ 6.231 ns      │ 6.233 ns      │ 100     │ 51200
│  ╰─ _3_bytes_x1000    6.892 µs      │ 8.014 µs      │ 6.902 µs      │ 6.917 µs      │ 100     │ 100
├─ default_hasher                     │               │               │               │         │
│  ├─ _100_bytes        30.8 ns       │ 30.96 ns      │ 30.89 ns      │ 30.88 ns      │ 100     │ 12800
│  ├─ _100_bytes_x1000  30.1 µs       │ 38.42 µs      │ 30.21 µs      │ 30.32 µs      │ 100     │ 100
│  ├─ _10_bytes         11.94 ns      │ 12.14 ns      │ 12.1 ns       │ 12.09 ns      │ 100     │ 25600
│  ├─ _10_bytes_x1000   14.92 µs      │ 15.34 µs      │ 14.93 µs      │ 14.94 µs      │ 100     │ 100
│  ├─ _30_bytes         14.37 ns      │ 20 ns         │ 14.44 ns      │ 14.64 ns      │ 100     │ 25600
│  ├─ _30_bytes_x1000   15.05 µs      │ 21.31 µs      │ 15.06 µs      │ 15.24 µs      │ 100     │ 100
│  ├─ _3_bytes          9.832 ns      │ 17.27 ns      │ 9.91 ns       │ 10.48 ns      │ 100     │ 25600
│  ╰─ _3_bytes_x1000    9.477 µs      │ 12.76 µs      │ 9.507 µs      │ 9.574 µs      │ 100     │ 100
├─ fasthash                           │               │               │               │         │
│  ├─ _100_bytes        19.46 ns      │ 35.89 ns      │ 19.8 ns       │ 20.95 ns      │ 100     │ 12800
│  ├─ _100_bytes_x1000  25.66 µs      │ 35.03 µs      │ 25.67 µs      │ 25.77 µs      │ 100     │ 100
│  ├─ _10_bytes         5.428 ns      │ 5.449 ns      │ 5.43 ns       │ 5.434 ns      │ 100     │ 51200
│  ├─ _10_bytes_x1000   15.24 µs      │ 16.82 µs      │ 15.25 µs      │ 15.34 µs      │ 100     │ 100
│  ├─ _30_bytes         9.438 ns      │ 9.481 ns      │ 9.481 ns      │ 9.47 ns       │ 100     │ 25600
│  ├─ _30_bytes_x1000   20.89 µs      │ 26.34 µs      │ 21.06 µs      │ 21.13 µs      │ 100     │ 100
│  ├─ _3_bytes          6.975 ns      │ 6.996 ns      │ 6.994 ns      │ 6.988 ns      │ 100     │ 51200
│  ╰─ _3_bytes_x1000    5.619 µs      │ 9.187 µs      │ 5.63 µs       │ 5.797 µs      │ 100     │ 100
├─ fnv                                │               │               │               │         │
│  ├─ _100_bytes        1.349 ns      │ 1.354 ns      │ 1.349 ns      │ 1.349 ns      │ 100     │ 204800
│  ├─ _100_bytes_x1000  99.24 µs      │ 105.5 µs      │ 100.5 µs      │ 100.8 µs      │ 100     │ 100
│  ├─ _10_bytes         1.08 ns       │ 1.407 ns      │ 1.08 ns       │ 1.134 ns      │ 100     │ 204800
│  ├─ _10_bytes_x1000   6.701 µs      │ 10.74 µs      │ 6.702 µs      │ 6.892 µs      │ 100     │ 100
│  ├─ _30_bytes         1.075 ns      │ 1.407 ns      │ 1.08 ns       │ 1.085 ns      │ 100     │ 204800
│  ├─ _30_bytes_x1000   24.86 µs      │ 32.16 µs      │ 24.87 µs      │ 24.96 µs      │ 100     │ 100
│  ├─ _3_bytes          1.08 ns       │ 1.525 ns      │ 1.08 ns       │ 1.087 ns      │ 100     │ 204800
│  ╰─ _3_bytes_x1000    2.714 µs      │ 4.718 µs      │ 2.965 µs      │ 3.027 µs      │ 100     │ 100
├─ fxhash                             │               │               │               │         │
│  ├─ _100_bytes        1.359 ns      │ 1.975 ns      │ 1.359 ns      │ 1.377 ns      │ 100     │ 102400
│  ├─ _100_bytes_x1000  15.37 µs      │ 20.09 µs      │ 15.38 µs      │ 15.45 µs      │ 100     │ 100
│  ├─ _10_bytes         1.08 ns       │ 1.085 ns      │ 1.08 ns       │ 1.081 ns      │ 100     │ 204800
│  ├─ _10_bytes_x1000   10.17 µs      │ 10.58 µs      │ 10.17 µs      │ 10.18 µs      │ 100     │ 100
│  ├─ _30_bytes         1.08 ns       │ 1.085 ns      │ 1.08 ns       │ 1.08 ns       │ 100     │ 204800
│  ├─ _30_bytes_x1000   13.11 µs      │ 19.31 µs      │ 13.38 µs      │ 13.43 µs      │ 100     │ 100
│  ├─ _3_bytes          1.08 ns       │ 1.525 ns      │ 1.08 ns       │ 1.097 ns      │ 100     │ 204800
│  ╰─ _3_bytes_x1000    1.892 µs      │ 2.714 µs      │ 1.893 µs      │ 1.911 µs      │ 100     │ 100
├─ gxhash                             │               │               │               │         │
│  ├─ _100_bytes        36.82 ns      │ 37.31 ns      │ 37 ns         │ 37.02 ns      │ 100     │ 6400
│  ├─ _100_bytes_x1000  56.6 µs       │ 65 µs         │ 57.36 µs      │ 57.79 µs      │ 100     │ 100
│  ├─ _10_bytes         11.35 ns      │ 59.53 ns      │ 11.47 ns      │ 11.93 ns      │ 100     │ 25600
│  ├─ _10_bytes_x1000   25.06 µs      │ 25.08 µs      │ 25.07 µs      │ 25.07 µs      │ 100     │ 100
│  ├─ _30_bytes         19.68 ns      │ 47.01 ns      │ 19.69 ns      │ 20.52 ns      │ 100     │ 12800
│  ├─ _30_bytes_x1000   37.72 µs      │ 43.45 µs      │ 37.73 µs      │ 37.8 µs       │ 100     │ 100
│  ├─ _3_bytes          11.43 ns      │ 18.32 ns      │ 11.51 ns      │ 11.78 ns      │ 100     │ 25600
│  ╰─ _3_bytes_x1000    24.54 µs      │ 29.25 µs      │ 24.54 µs      │ 24.61 µs      │ 100     │ 100
├─ highway                            │               │               │               │         │
│  ├─ _100_bytes        41.68 ns      │ 72.21 ns      │ 41.68 ns      │ 42.68 ns      │ 100     │ 6400
│  ├─ _100_bytes_x1000  46.49 µs      │ 79.14 µs      │ 46.96 µs      │ 51.43 µs      │ 100     │ 100
│  ├─ _10_bytes         36.21 ns      │ 83.81 ns      │ 42.95 ns      │ 43.7 ns       │ 100     │ 6400
│  ├─ _10_bytes_x1000   36.79 µs      │ 48.77 µs      │ 36.93 µs      │ 37.67 µs      │ 100     │ 100
│  ├─ _30_bytes         36.36 ns      │ 55.62 ns      │ 36.51 ns      │ 37.37 ns      │ 100     │ 6400
│  ├─ _30_bytes_x1000   36.14 µs      │ 42.88 µs      │ 36.15 µs      │ 36.36 µs      │ 100     │ 100
│  ├─ _3_bytes          36.36 ns      │ 56.71 ns      │ 36.51 ns      │ 37.34 ns      │ 100     │ 6400
│  ╰─ _3_bytes_x1000    35.74 µs      │ 57.25 µs      │ 35.75 µs      │ 36.77 µs      │ 100     │ 100
├─ hud_slice_by_8                     │               │               │               │         │
│  ├─ _100_bytes        48.71 ns      │ 60.17 ns      │ 48.89 ns      │ 49.13 ns      │ 100     │ 6400
│  ├─ _100_bytes_x1000  44.53 µs      │ 56.43 µs      │ 44.72 µs      │ 45.74 µs      │ 100     │ 100
│  ├─ _10_bytes         7.992 ns      │ 19.53 ns      │ 8.07 ns       │ 8.404 ns      │ 100     │ 25600
│  ├─ _10_bytes_x1000   13.88 µs      │ 14.34 µs      │ 13.91 µs      │ 13.92 µs      │ 100     │ 100
│  ├─ _30_bytes         25.56 ns      │ 38.25 ns      │ 25.71 ns      │ 26.44 ns      │ 100     │ 6400
│  ├─ _30_bytes_x1000   28.18 µs      │ 34.96 µs      │ 28.28 µs      │ 28.36 µs      │ 100     │ 100
│  ├─ _3_bytes          6.934 ns      │ 11.32 ns      │ 6.973 ns      │ 7.156 ns      │ 100     │ 25600
│  ╰─ _3_bytes_x1000    6.02 µs       │ 10.11 µs      │ 6.031 µs      │ 6.121 µs      │ 100     │ 100
├─ meowhash                           │               │               │               │         │
│  ├─ _100_bytes        123.6 ns      │ 225.7 ns      │ 123.7 ns      │ 128.6 ns      │ 100     │ 1600
│  ├─ _100_bytes_x1000  126.7 µs      │ 215.2 µs      │ 128.6 µs      │ 132.5 µs      │ 100     │ 100
│  ├─ _10_bytes         108.9 ns      │ 376 ns        │ 155 ns        │ 157.3 ns      │ 100     │ 3200
│  ├─ _10_bytes_x1000   110.4 µs      │ 142.7 µs      │ 110.4 µs      │ 111.7 µs      │ 100     │ 100
│  ├─ _30_bytes         108.3 ns      │ 192.9 ns      │ 110.2 ns      │ 116.5 ns      │ 100     │ 3200
│  ├─ _30_bytes_x1000   108.9 µs      │ 162.4 µs      │ 108.9 µs      │ 111.3 µs      │ 100     │ 100
│  ├─ _3_bytes          107.4 ns      │ 245.5 ns      │ 108.6 ns      │ 112.8 ns      │ 100     │ 3200
│  ╰─ _3_bytes_x1000    108.4 µs      │ 159.8 µs      │ 109.8 µs      │ 111.3 µs      │ 100     │ 100
├─ rustc_hash                         │               │               │               │         │
│  ├─ _100_bytes        1.329 ns      │ 1.975 ns      │ 1.359 ns      │ 1.442 ns      │ 100     │ 102400
│  ├─ _100_bytes_x1000  13.11 µs      │ 20.06 µs      │ 13.12 µs      │ 13.2 µs       │ 100     │ 100
│  ├─ _10_bytes         1.08 ns       │ 1.393 ns      │ 1.08 ns       │ 1.083 ns      │ 100     │ 204800
│  ├─ _10_bytes_x1000   10.67 µs      │ 11.07 µs      │ 10.68 µs      │ 10.69 µs      │ 100     │ 100
│  ├─ _30_bytes         1.08 ns       │ 1.402 ns      │ 1.08 ns       │ 1.089 ns      │ 100     │ 204800
│  ├─ _30_bytes_x1000   10.43 µs      │ 11.1 µs       │ 10.64 µs      │ 10.64 µs      │ 100     │ 100
│  ├─ _3_bytes          1.08 ns       │ 4.079 ns      │ 1.08 ns       │ 1.117 ns      │ 100     │ 204800
│  ╰─ _3_bytes_x1000    2.864 µs      │ 4.387 µs      │ 2.875 µs      │ 2.951 µs      │ 100     │ 100
├─ wyhash                             │               │               │               │         │
│  ├─ _100_bytes        11.08 ns      │ 18.48 ns      │ 11.12 ns      │ 11.25 ns      │ 100     │ 25600
│  ├─ _100_bytes_x1000  15.08 µs      │ 19.6 µs       │ 15.28 µs      │ 15.53 µs      │ 100     │ 100
│  ├─ _10_bytes         5.818 ns      │ 8.443 ns      │ 5.82 ns       │ 5.919 ns      │ 100     │ 51200
│  ├─ _10_bytes_x1000   10.3 µs       │ 19.55 µs      │ 10.4 µs       │ 10.56 µs      │ 100     │ 100
│  ├─ _30_bytes         9.242 ns      │ 11.71 ns      │ 9.281 ns      │ 9.378 ns      │ 100     │ 25600
│  ├─ _30_bytes_x1000   10.56 µs      │ 14.04 µs      │ 10.62 µs      │ 10.81 µs      │ 100     │ 100
│  ├─ _3_bytes          7.012 ns      │ 9.598 ns      │ 8.578 ns      │ 8.487 ns      │ 100     │ 25600
│  ╰─ _3_bytes_x1000    5.519 µs      │ 9.918 µs      │ 6.862 µs      │ 6.856 µs      │ 100     │ 100
├─ xxh3                               │               │               │               │         │
│  ├─ _100_bytes        2.17 ns       │ 5.82 ns       │ 2.171 ns      │ 2.432 ns      │ 100     │ 102400
│  ├─ _100_bytes_x1000  13.72 µs      │ 29.93 µs      │ 14.05 µs      │ 14.08 µs      │ 100     │ 100
│  ├─ _10_bytes         2.17 ns       │ 2.181 ns      │ 2.171 ns      │ 2.173 ns      │ 100     │ 102400
│  ├─ _10_bytes_x1000   11.1 µs       │ 11.52 µs      │ 11.27 µs      │ 11.27 µs      │ 100     │ 100
│  ├─ _30_bytes         2.17 ns       │ 2.181 ns      │ 2.171 ns      │ 2.173 ns      │ 100     │ 102400
│  ├─ _30_bytes_x1000   11.18 µs      │ 11.37 µs      │ 11.18 µs      │ 11.18 µs      │ 100     │ 100
│  ├─ _3_bytes          2.17 ns       │ 7.493 ns      │ 2.171 ns      │ 2.225 ns      │ 100     │ 102400
│  ╰─ _3_bytes_x1000    3.175 µs      │ 3.395 µs      │ 3.195 µs      │ 3.197 µs      │ 100     │ 100
╰─ xxhash_rust_xxh3                   │               │               │               │         │
   ├─ _100_bytes        28.21 ns      │ 28.3 ns       │ 28.3 ns       │ 28.28 ns      │ 100     │ 12800
   ├─ _100_bytes_x1000  30.88 µs      │ 35.57 µs      │ 30.88 µs      │ 30.95 µs      │ 100     │ 100
   ├─ _10_bytes         24.78 ns      │ 24.86 ns      │ 24.78 ns      │ 24.78 ns      │ 100     │ 12800
   ├─ _10_bytes_x1000   25.13 µs      │ 32.5 µs       │ 25.14 µs      │ 25.33 µs      │ 100     │ 100
   ├─ _30_bytes         25.25 ns      │ 25.33 ns      │ 25.32 ns      │ 25.31 ns      │ 100     │ 12800
   ├─ _30_bytes_x1000   28.49 µs      │ 35.69 µs      │ 28.87 µs      │ 28.87 µs      │ 100     │ 100
   ├─ _3_bytes          17.57 ns      │ 34.17 ns      │ 17.89 ns      │ 20.04 ns      │ 100     │ 12800
   ╰─ _3_bytes_x1000    17.42 µs      │ 32.5 µs       │ 17.66 µs      │ 19.29 µs      │ 100     │ 100
```

As you can see, the default `stringA == stringB` is about 10^6 times faster than the fastest hashing algorithm. So yeah, hashing is slower than comparing (at least in Rust). This was tested on the `article*.txt` files you see on the file tree.


## Running the benchmarks yourself

(Git and Cargo must be installed)

Just clone this repo with `git clone https://github.com/rosetta-rs/hashing-rosetta-rs` or download the ZIP with the green button, and run `cargo bench`.