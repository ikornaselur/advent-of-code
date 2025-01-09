# Year 2024

How mad can we go with performance? There's no need for it, all the solutions
tend to run within a second.. and most within 1ms, but how fast is fast enough?

Let's see how fast we can get each solution.. 

This is running on an Intel i5-12400. The values are the median runtime from
the homebrew speed test functionality in this repo.

Day | Original part 1 | Original part 2 | Optimised part 1 | Optimised part 2
--- | --- | --- | --- | ---
1 | 45µs | 62µs | 45µs | 43µs
2 | 140µs | 220µs | |
3 | 115µs | 115µs | |
4 | 700µs | 295µs | |
5 | 270µs | 420µs | |
6 | 310µs | **410ms** | |
7 | 230µs | 315µs | |
8 | 35µs | 75µs | |
9 | **10ms** | **35ms** | |
10 | 190µs | 165µs | |
11 | 175µs | **11.5ms** | |
12 | **5ms** | **6ms** | |
13 | 32µs | 32µs | |
14 | 30µs | **45ms** | |
15 | 700µs | **1.1ms** | |
16 | **2.7ms** | **555ms** | |
17 | 2.2µs | 280µs | |
18 | 745µs | **675ms** | |
19 | **30ms** | **29ms** | |
20 | 977µs | **50ms** | |
21 | 145µs | **1.5ms** | |
22 | 30µs | **160ms** | |
