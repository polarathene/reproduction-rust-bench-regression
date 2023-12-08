# Reproduction for regression in bench perf timing

1. Run `cargo bench` a few times, observe the `fastest` timing column.
2. In `lib.rs` remove `fold_copy()` and repeat step 1 to compare timing results.
3. Timing should have decreased, verifying the regression.

## Output examples

### With regression present

```
     Running benches/criterion.rs (target/release/deps/criterion-a56b6d24776194ad)
Gnuplot not found, using plotters backend
criterion/into_string_fold
                        time:   [133.56 ns 135.63 ns 137.84 ns]
                        change: [-4.5839% -2.5971% -0.6088%] (p = 0.01 < 0.05)
                        Change within noise threshold.

     Running benches/divan.rs (target/release/deps/divan-e88aec68da77a7cc)
Timer precision: 21 ns
divan                fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ into_string_fold  137.2 ns      │ 621.2 ns      │ 159.4 ns      │ 156.2 ns      │ 1000    │ 128000
```

### Without regression

```
     Running benches/criterion.rs (target/release/deps/criterion-a56b6d24776194ad)
Gnuplot not found, using plotters backend
criterion/into_string_fold
                        time:   [107.28 ns 110.38 ns 113.99 ns]
                        change: [-18.487% -15.598% -12.446%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) high mild

     Running benches/divan.rs (target/release/deps/divan-e88aec68da77a7cc)
Timer precision: 21 ns
divan                fastest       │ slowest       │ median        │ mean          │ samples │ iters
╰─ into_string_fold  102.1 ns      │ 290.8 ns      │ 107.6 ns      │ 113.4 ns      │ 1000    │ 128000
```

## Description

This is a minimized and isolated reproduction I observed where just having a copy of a function (as dead code) was introducing a consistent bump in timings for the actual benched method.

On my system the difference is approx `100ns` vs `135ns`, this is consistent for me just by removing the copied function from source.

I suspect it ~~may be environment specific~~ (_**UPDATE:** confirmed reproduction by other users_), or vary by rust release with the compiler [optimizing differently][criterion::docs::faq], so I've put this reproduction together for verification.

The comments in `lib.rs` show where changes can be made to avoid the regression.

[criterion::docs::faq]: https://bheisler.github.io/criterion.rs/book/faq.html#i-made-a-trivial-change-to-my-source-and-criterionrs-reports-a-large-change-in-performance-why

## System Details

- Platform: x86_64 GNU/Linux
- CPU: AMD Ryzen 9 7940HS
- Linux Kernel: 5.15.123.1-microsoft-standard-WSL2 (Host: Windows 11 23H2)
- WSL2 OS: Ubuntu 22.04.2 LTS (Jammy Jellyfish)

**Reproduced with:**
- cargo 1.74.0 (ecb9851af 2023-10-18)
- cargo 1.76.0-nightly (71cd3a926 2023-11-20)
