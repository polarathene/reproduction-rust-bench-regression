# Reproduction for regression in bench perf timing

1. Run `cargo bench` a few times, observe the `fastest` timing column.
2. In `lib.rs` remove `fold_copy()` and repeat step 1 to compare timing results.
3. Timing should have decreased, verifying the regression.

## Description

This is a minimized and isolated reproduction I observed where just having a copy of a function (as dead code) was introducing a consistent bump in timings for the actual benched method.

On my system the difference is approx `100ns` vs `135ns`, this is consistent for me just by removing the copied function from source.

I suspect it may be environment specific, or vary by rust release with the compiler, so I've put this reproduction together for verification.

The comments in `lib.rs` show where changes can be made to avoid the regression.

## System Details

- Platform: x86_64 GNU/Linux
- CPU: AMD Ryzen 9 7940HS
- Linux Kernel: 5.15.123.1-microsoft-standard-WSL2 (Host: Windows 11 23H2)
- WSL2 OS: Ubuntu 22.04.2 LTS (Jammy Jellyfish)

**Reproduced with:**
- cargo 1.74.0 (ecb9851af 2023-10-18)
- cargo 1.76.0-nightly (71cd3a926 2023-11-20)
