# String comparison experiment

What's faster, hashing a string and comparing the hash, or simply comparing the string? This Rust benchmark measures just that.
 
Here are the results in a benchmark done in my machine:

plain
```
meow                 fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ comparison_long   0.002 ns      │ 0.183 ns      │ 0.07 ns       │ 0.048 ns      │ 100     │ 819200
├─ comparison_short  0.069 ns      │ 0.07 ns       │ 0.07 ns       │ 0.07 ns       │ 100     │ 819200
├─ comparison_true   0.002 ns      │ 0.071 ns      │ 0.003 ns      │ 0.034 ns      │ 100     │ 819200
├─ ahash                           │               │               │               │         │
│  ├─ equal          499.4 µs      │ 814.6 µs      │ 516.1 µs      │ 518.1 µs      │ 100     │ 100
│  ├─ long           509 µs        │ 796.8 µs      │ 527.4 µs      │ 541.6 µs      │ 100     │ 100
│  ╰─ short          10.06 µs      │ 26 µs         │ 10.24 µs      │ 10.53 µs      │ 100     │ 100
├─ default_hasher                  │               │               │               │         │
│  ├─ equal          499.7 µs      │ 872.6 µs      │ 510.2 µs      │ 525.1 µs      │ 100     │ 100
│  ├─ long           514 µs        │ 869.4 µs      │ 532.8 µs      │ 550.1 µs      │ 100     │ 100
│  ╰─ short          9.807 µs      │ 25.19 µs      │ 9.837 µs      │ 10.1 µs       │ 100     │ 100
├─ fasthash                        │               │               │               │         │
│  ├─ equal          648.8 µs      │ 984.5 µs      │ 650.1 µs      │ 674.4 µs      │ 100     │ 100
│  ├─ long           707.1 µs      │ 952.4 µs      │ 739.2 µs      │ 758.4 µs      │ 100     │ 100
│  ╰─ short          13.3 µs       │ 18.48 µs      │ 13.32 µs      │ 13.47 µs      │ 100     │ 100
├─ fxhash                          │               │               │               │         │
│  ├─ equal          1.08 ms       │ 1.164 ms      │ 1.081 ms      │ 1.086 ms      │ 100     │ 100
│  ├─ long           1.083 ms      │ 1.125 ms      │ 1.089 ms      │ 1.093 ms      │ 100     │ 100
│  ╰─ short          21.19 µs      │ 29.17 µs      │ 21.2 µs       │ 21.47 µs      │ 100     │ 100
├─ meowhash                        │               │               │               │         │
│  ├─ equal          880.9 µs      │ 1.397 ms      │ 916.2 µs      │ 1.099 ms      │ 100     │ 100
│  ├─ long           901.5 µs      │ 1.391 ms      │ 969.7 µs      │ 1.114 ms      │ 100     │ 100
│  ╰─ short          17.52 µs      │ 30.82 µs      │ 17.65 µs      │ 19.7 µs       │ 100     │ 100
╰─ xxh3                            │               │               │               │         │
   ├─ equal          356 µs        │ 813.3 µs      │ 422.9 µs      │ 412.6 µs      │ 100     │ 100
   ├─ long           376.2 µs      │ 767.9 µs      │ 455.7 µs      │ 459.1 µs      │ 100     │ 100
   ╰─ short          6.801 µs      │ 22.18 µs      │ 6.851 µs      │ 7.002 µs      │ 100     │ 100
```

As you can see, the default `stringA == stringB` is about 10^6 times faster than the fastest hashing algorithm (`xxh3`). So yeah, hashing is slower than comparing (at least in Rust). This was tested on the `article*.txt` files you see on the file tree.