# Purpose
Demonstrate a recursive approach to finding a knight's tour, using backtracking.

# Results
A release build without path optimization produces this result:
```
Solution found
Length of tour: 64
00 07 12 27 50 31 40 29
11 04 09 32 39 28 51 48
08 01 06 13 26 49 30 41
05 10 03 58 33 38 47 52
02 17 14 25 46 59 42 37
21 24 19 16 57 34 53 60
18 15 22 45 62 55 36 43
23 20 63 56 35 44 61 54
cargo run --release  47.34s user 0.03s system 99% cpu 47.487 total
```

A release build with the path optimization produces this result:
```
Solution found
Length of tour: 64
00 29 14 41 32 27 12 39
15 42 31 28 13 40 35 26
30 01 52 43 46 33 38 11
53 16 45 48 51 36 25 34
02 49 54 59 44 47 10 37
17 56 19 50 63 60 07 24
20 03 58 55 22 05 62 09
57 18 21 04 61 08 23 06
cargo run --release  0.06s user 0.02s system 9% cpu 0.735 total
```

Optimizaed code is about 750 times faster.

# Limitations
1. Code does not attempt to find cyclical tours but only any tour, regardless of where it ends

# Optimizations
1. Code uses optimal approach of trying shortest paths first
