# String comparison experiment

What's faster, hashing a string and comparing the hash, or simply comparing the string? This Rust benchmark measures just that.
 
Here are the results in a benchmark done in my machine:

```plain
Timer precision: 30 ns
meow                 fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ comparison_long   0.002 ns      │ 0.287 ns      │ 0.004 ns      │ 0.006 ns      │ 100     │ 819200
├─ comparison_short  0.003 ns      │ 0.003 ns      │ 0.003 ns      │ 0.003 ns      │ 100     │ 819200
├─ comparison_true   0.003 ns      │ 0.003 ns      │ 0.003 ns      │ 0.003 ns      │ 100     │ 819200
├─ ahash                           │               │               │               │         │
│  ├─ equal          232 µs        │ 678.3 µs      │ 241.4 µs      │ 247.7 µs      │ 100     │ 100
│  ├─ long           227.9 µs      │ 866.1 µs      │ 295.2 µs      │ 316 µs        │ 100     │ 100
│  ╰─ short          3.145 µs      │ 24.28 µs      │ 3.175 µs      │ 3.383 µs      │ 100     │ 100
├─ blake3                          │               │               │               │         │
│  ├─ equal          2.671 ms      │ 3.443 ms      │ 2.785 ms      │ 2.83 ms       │ 100     │ 100
│  ├─ long           2.723 ms      │ 4.575 ms      │ 2.844 ms      │ 2.953 ms      │ 100     │ 100
│  ╰─ short          53.07 µs      │ 86.7 µs       │ 59.23 µs      │ 60.42 µs      │ 100     │ 100
├─ cityhasher                      │               │               │               │         │
│  ├─ equal          391.6 µs      │ 524.1 µs      │ 392.8 µs      │ 399.6 µs      │ 100     │ 100
│  ├─ long           407.8 µs      │ 714.2 µs      │ 415.3 µs      │ 437.6 µs      │ 100     │ 100
│  ╰─ short          7.744 µs      │ 13.85 µs      │ 7.764 µs      │ 7.9 µs        │ 100     │ 100
├─ default_hasher                  │               │               │               │         │
│  ├─ equal          1.405 ms      │ 1.646 ms      │ 1.415 ms      │ 1.423 ms      │ 100     │ 100
│  ├─ long           1.417 ms      │ 1.971 ms      │ 1.44 ms       │ 1.463 ms      │ 100     │ 100
│  ╰─ short          27.64 µs      │ 37.48 µs      │ 27.72 µs      │ 27.99 µs      │ 100     │ 100
├─ fasthash                        │               │               │               │         │
│  ├─ equal          666.1 µs      │ 782.3 µs      │ 667.2 µs      │ 678.3 µs      │ 100     │ 100
│  ├─ long           722.4 µs      │ 903 µs        │ 743.6 µs      │ 760.5 µs      │ 100     │ 100
│  ╰─ short          13.65 µs      │ 18.25 µs      │ 13.66 µs      │ 13.78 µs      │ 100     │ 100
├─ fnv                             │               │               │               │         │
│  ├─ equal          7.002 ms      │ 11.12 ms      │ 7.133 ms      │ 7.205 ms      │ 100     │ 100
│  ├─ long           7.088 ms      │ 7.67 ms       │ 7.147 ms      │ 7.177 ms      │ 100     │ 100
│  ╰─ short          139.2 µs      │ 169.8 µs      │ 139.2 µs      │ 141.3 µs      │ 100     │ 100
├─ fxhash                          │               │               │               │         │
│  ├─ equal          1.109 ms      │ 1.228 ms      │ 1.116 ms      │ 1.123 ms      │ 100     │ 100
│  ├─ long           1.116 ms      │ 1.232 ms      │ 1.136 ms      │ 1.14 ms       │ 100     │ 100
│  ╰─ short          21.76 µs      │ 34.18 µs      │ 21.76 µs      │ 22.13 µs      │ 100     │ 100
├─ gxhash                          │               │               │               │         │
│  ├─ equal          117.5 µs      │ 375.4 µs      │ 118.5 µs      │ 125.1 µs      │ 100     │ 100
│  ├─ long           173.7 µs      │ 358.4 µs      │ 210.4 µs      │ 222.2 µs      │ 100     │ 100
│  ╰─ short          2.223 µs      │ 9.367 µs      │ 2.424 µs      │ 2.527 µs      │ 100     │ 100
├─ highway                         │               │               │               │         │
│  ├─ equal          563.3 µs      │ 638.9 µs      │ 568.7 µs      │ 572.6 µs      │ 100     │ 100
│  ├─ long           574.5 µs      │ 719.2 µs      │ 606.7 µs      │ 617.8 µs      │ 100     │ 100
│  ╰─ short          11.12 µs      │ 24.27 µs      │ 11.2 µs       │ 11.36 µs      │ 100     │ 100
├─ hud_slice_by_8                  │               │               │               │         │
│  ├─ equal          2.615 ms      │ 3.064 ms      │ 2.659 ms      │ 2.683 ms      │ 100     │ 100
│  ├─ long           2.626 ms      │ 3.176 ms      │ 2.666 ms      │ 2.701 ms      │ 100     │ 100
│  ╰─ short          51.08 µs      │ 63.11 µs      │ 51.44 µs      │ 52.05 µs      │ 100     │ 100
├─ meowhash                        │               │               │               │         │
│  ├─ equal          188.4 µs      │ 376.9 µs      │ 189.1 µs      │ 195.1 µs      │ 100     │ 100
│  ├─ long           200.5 µs      │ 450.1 µs      │ 210.8 µs      │ 225.2 µs      │ 100     │ 100
│  ╰─ short          3.325 µs      │ 9.657 µs      │ 3.345 µs      │ 3.415 µs      │ 100     │ 100
├─ rustc_hash                      │               │               │               │         │
│  ├─ equal          335.1 µs      │ 460.3 µs      │ 355.3 µs      │ 357.3 µs      │ 100     │ 100
│  ├─ long           351.2 µs      │ 566.5 µs      │ 388 µs        │ 406.7 µs      │ 100     │ 100
│  ╰─ short          6.592 µs      │ 9.798 µs      │ 6.612 µs      │ 6.647 µs      │ 100     │ 100
├─ wyhash                          │               │               │               │         │
│  ├─ equal          421.9 µs      │ 596.4 µs      │ 449.6 µs      │ 458.4 µs      │ 100     │ 100
│  ├─ long           427.8 µs      │ 769.3 µs      │ 477.3 µs      │ 503.2 µs      │ 100     │ 100
│  ╰─ short          8.025 µs      │ 13.03 µs      │ 8.165 µs      │ 8.219 µs      │ 100     │ 100
├─ xxh3                            │               │               │               │         │
│  ├─ equal          434.2 µs      │ 721.1 µs      │ 454.5 µs      │ 478.8 µs      │ 100     │ 100
│  ├─ long           445.8 µs      │ 863.3 µs      │ 483.4 µs      │ 507.5 µs      │ 100     │ 100
│  ╰─ short          8.535 µs      │ 13.68 µs      │ 8.545 µs      │ 8.595 µs      │ 100     │ 100
╰─ xxhash_rust_xxh3                │               │               │               │         │
   ├─ equal          563.4 µs      │ 936.2 µs      │ 611.7 µs      │ 609.3 µs      │ 100     │ 100
   ├─ long           621.4 µs      │ 1.013 ms      │ 656.1 µs      │ 686.5 µs      │ 100     │ 100
   ╰─ short          12 µs         │ 34.01 µs      │ 12.05 µs      │ 12.63 µs      │ 100     │ 100
```

As you can see, the default `stringA == stringB` is about 10^6 times faster than the fastest hashing algorithm. So yeah, hashing is slower than comparing (at least in Rust). This was tested on the `article*.txt` files you see on the file tree.
