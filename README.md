# String comparison experiment

What's faster, hashing a string and comparing the hash, or simply comparing the string? This Rust benchmark measures just that.
 
Here are the results in a benchmark done in my machine:

```plain
meow                 fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ comparison_long   0.004 ns      │ 0.008 ns      │ 0.004 ns      │ 0.005 ns      │ 100     │ 819200
├─ comparison_short  0.003 ns      │ 12.08 ns      │ 0.004 ns      │ 0.167 ns      │ 100     │ 819200
├─ comparison_true   0.001 ns      │ 0.001 ns      │ 0.001 ns      │ 0.001 ns      │ 100     │ 1638400
├─ ahash                           │               │               │               │         │
│  ├─ equal          480.2 µs      │ 807 µs        │ 512.8 µs      │ 524.2 µs      │ 100     │ 100
│  ├─ long           522 µs        │ 840.2 µs      │ 535.4 µs      │ 553.3 µs      │ 100     │ 100
│  ╰─ short          10.05 µs      │ 26.93 µs      │ 10.09 µs      │ 10.26 µs      │ 100     │ 100
├─ default_hasher                  │               │               │               │         │
│  ├─ equal          1.414 ms      │ 1.925 ms      │ 1.428 ms      │ 1.453 ms      │ 100     │ 100
│  ├─ long           1.458 ms      │ 1.764 ms      │ 1.487 ms      │ 1.517 ms      │ 100     │ 100
│  ╰─ short          28.61 µs      │ 42.99 µs      │ 29.17 µs      │ 29.27 µs      │ 100     │ 100
├─ fasthash                        │               │               │               │         │
│  ├─ equal          721.4 µs      │ 1.017 ms      │ 721.9 µs      │ 740.6 µs      │ 100     │ 100
│  ├─ long           766.1 µs      │ 1.237 ms      │ 791.3 µs      │ 833.2 µs      │ 100     │ 100
│  ╰─ short          14.19 µs      │ 29.59 µs      │ 14.2 µs       │ 14.38 µs      │ 100     │ 100
├─ fnv                             │               │               │               │         │
│  ├─ equal          7.105 ms      │ 8.608 ms      │ 7.196 ms      │ 7.231 ms      │ 100     │ 100
│  ├─ long           7.115 ms      │ 8.105 ms      │ 7.197 ms      │ 7.229 ms      │ 100     │ 100
│  ╰─ short          139.2 µs      │ 163.5 µs      │ 139.2 µs      │ 139.9 µs      │ 100     │ 100
├─ fxhash                          │               │               │               │         │
│  ├─ equal          1.109 ms      │ 1.507 ms      │ 1.115 ms      │ 1.145 ms      │ 100     │ 100
│  ├─ long           1.113 ms      │ 1.229 ms      │ 1.128 ms      │ 1.134 ms      │ 100     │ 100
│  ╰─ short          21.76 µs      │ 46.67 µs      │ 21.76 µs      │ 22.09 µs      │ 100     │ 100
├─ meowhash                        │               │               │               │         │
│  ├─ equal          893.8 µs      │ 1.402 ms      │ 905.6 µs      │ 1.013 ms      │ 100     │ 100
│  ├─ long           900.1 µs      │ 1.417 ms      │ 981.3 µs      │ 1.096 ms      │ 100     │ 100
│  ╰─ short          26.23 µs      │ 52.19 µs      │ 26.78 µs      │ 26.86 µs      │ 100     │ 100
├─ rustc_hash                      │               │               │               │         │
│  ├─ equal          333.3 µs      │ 448.6 µs      │ 348 µs        │ 350.6 µs      │ 100     │ 100
│  ├─ long           353 µs        │ 609.1 µs      │ 368.2 µs      │ 385.4 µs      │ 100     │ 100
│  ╰─ short          6.551 µs      │ 10.53 µs      │ 6.562 µs      │ 6.604 µs      │ 100     │ 100
╰─ xxh3                            │               │               │               │         │
   ├─ equal          343.8 µs      │ 524.3 µs      │ 423.4 µs      │ 405 µs        │ 100     │ 100
   ├─ long           370.8 µs      │ 550.7 µs      │ 389.9 µs      │ 408.7 µs      │ 100     │ 100
   ╰─ short          6.792 µs      │ 20.25 µs      │ 6.852 µs      │ 7.123 µs      │ 100     │ 100
```

As you can see, the default `stringA == stringB` is about 10^6 times faster than the fastest hashing algorithm (`xxh3`). So yeah, hashing is slower than comparing (at least in Rust). This was tested on the `article*.txt` files you see on the file tree.
