# Where bevy_gooey_loop's time goes

Written by `tools/loop/steps.py publish` after every pass; the rows are in [`bevy_gooey_loop.tsv`](bevy_gooey_loop.tsv). Updated 2026-11-26 03:00Z.

## What the last passes shipped

From [`bevy_gooey_loop-audit.tsv`](bevy_gooey_loop-audit.tsv): commits on main, unpushed commits, package files changed, release tags, doc and demo files, and what was missing.

| pass | at | commits | unpushed | package_files | released | docs_files | demo_files | missing |
|---|---|---|---|---|---|---|---|---|
| 345 | 2026-11-22T09:40Z | 7 | 0 | 8 | v0.53.0,v0.54.0 | 6 | 1 | - |
| 346 | 2026-11-22T10:21Z | 7 | 0 | 6 | v0.55.0 | 8 | 1 | - |
| 347 | 2026-11-22T14:04Z | 8 | 0 | 4 | v0.56.0 | 6 | 1 | - |
| 348 | 2026-11-22T15:12Z | 7 | 0 | 6 | v0.57.0 | 6 | 1 | - |
| 349 | 2026-11-22T15:29Z | 0 | 0 | 0 | - | 0 | 0 | nothing shipped |
| 350 | 2026-11-23T00:06Z | 5 | 0 | 5 | v0.58.0 | 6 | 1 | - |
| 351 | 2026-11-23T00:30Z | 4 | 0 | 6 | - | 8 | 1 | release |
| 352 | 2026-11-23T15:35Z | 0 | 0 | 0 | - | 0 | 0 | nothing shipped |
| 353 | 2026-11-25T23:47Z | 3 | 0 | 3 | v0.59.0 | 6 | 1 | - |
| 354 | 2026-11-26T00:04Z | 0 | 0 | 0 | - | 0 | 0 | nothing shipped |
| 355 | 2026-11-26T01:05Z | 5 | 0 | 3 | v0.60.0 | 10 | 2 | - |
| 356 | 2026-11-26T03:00Z | 15 | 0 | 4 | v0.61.0 | 6 | 1 | - |

## How each configuration did

From [`bevy_gooey_loop-config.tsv`](bevy_gooey_loop-config.tsv): one row per pass with its host, wall time, model, thinking level and build settings.

```text
host       model                        think    jobs  threads  passes  mean min  median min
big        anthropic/claude-sonnet-5-5  high     8     8            11      41.7        36.4
big        anthropic/claude-sonnet-5-5  high     6     4            34      31.9        28.5
big        anthropic/claude-sonnet-5-5  high     3     4             8      37.0        33.9
big        ollama-cloud/deepseek-v4.1-flash:1017 high     3     4             5      58.1        60.4
```

## The last 12 passes

```text
12 pass(es), 4775 steps, 9.3 h of pass time

category                       hours  share  steps  mean s
wait: other                     3.82  41.1%    238    57.8
model                           2.79  30.0%   2195     4.6
cargo test                      1.16  12.4%    146    28.5
cargo build                     0.54   5.7%    124    15.5
shell                           0.50   5.4%   1316     1.4
cargo publish                   0.13   1.4%     21    23.0
phase mark                      0.09   1.0%     84     4.0
cargo clippy                    0.04   0.5%     16    10.0
github                          0.04   0.4%     58     2.6
git                             0.04   0.4%    112     1.2
remote wait                     0.04   0.4%      8    15.8
push gate                       0.03   0.3%     22     4.8
homestill                       0.03   0.3%     37     2.6
web                             0.02   0.3%     11     7.8
read files                      0.02   0.3%    240     0.4

phase                          hours  share  steps  mean s
research                        3.28  35.2%   1772     6.7
build                           1.80  19.4%   1118     5.8
gate                            1.21  13.0%    162    26.9
prove                           1.01  10.8%    400     9.1
bugs                            0.63   6.8%    383     6.0
choose                          0.43   4.6%    329     4.7
orient                          0.41   4.4%    433     3.4
release                         0.27   2.9%     68    14.1
ship                            0.20   2.2%     38    19.3
log                             0.06   0.7%     40     5.4
done                            0.00   0.0%      2     3.8

slowest single steps
  pass  356     904s  bg    cargo build          bg_41 completed: CARGO_INCREMENTAL=0 RUSTC_WRAPPER= CARGO_BUILD_JOBS=2 timeout 900 cargo b
  pass  353     735s  bg    shell                bg_18 completed: cp /tmp/g117-fix.rs src/code_overlay.rs && export CARGO_TARGET_DIR=$HOME/
  pass  356     594s  model model                
  pass  353     300s  bg    shell                bg_8 failed: export CARGO_TARGET_DIR=$HOME/.cargo-target-shared/global && unset CARGO_INCR
  pass  356     300s  bg    cargo build          bg_20 failed: CARGO_INCREMENTAL=0 RUSTC_WRAPPER= cargo build --release --lib 2>&1 | tail -
  pass  356     247s  tool  wait: other          {}
  pass  352     242s  bg    cargo test           bg_9 completed: RUSTC_WRAPPER="" CARGO_BUILD_RUSTC_WRAPPER="" CARGO_INCREMENTAL=0 CARGO_BU
  pass  353     242s  bg    shell                bg_11 completed: export CARGO_TARGET_DIR=$HOME/.cargo-target-shared/global && unset CARGO_
  pass  356     240s  tool  wait: other          {}
  pass  349     240s  tool  wait: other          {}
  pass  352     240s  tool  wait: other          {}
  pass  356     240s  bg    cargo build          bg_23 completed: CARGO_INCREMENTAL=0 RUSTC_WRAPPER= timeout 240 cargo build --release --li
  pass  348     240s  tool  wait: other          {}
  pass  353     240s  tool  wait: other          {}
  pass  353     240s  tool  wait: other          {}
```

## Every pass recorded

```text
76 pass(es), 22654 steps, 58.7 h of pass time

category                       hours  share  steps  mean s
model                          22.25  37.9%  10325     7.8
wait: gate                      9.91  16.9%     74   482.3
wait: other                     9.77  16.6%    486    72.3
cargo test                      3.81   6.5%    691    19.9
wait: tests                     2.85   4.9%     34   302.0
shell                           2.49   4.2%   4335     2.1
cargo build                     2.16   3.7%    468    16.6
wait: build                     1.16   2.0%     21   199.4
phase mark                      1.05   1.8%    543     7.0
wait: clip                      0.79   1.3%     30    94.9
cargo publish                   0.35   0.6%     83    15.2
remote wait                     0.35   0.6%     47    26.8
read files                      0.29   0.5%   2396     0.4
github                          0.24   0.4%    251     3.5
cargo clippy                    0.23   0.4%     93     9.1

phase                          hours  share  steps  mean s
orient                         19.48  33.2%   4976    14.1
prove                           9.92  16.9%   4745     7.5
gate                            9.38  16.0%   1418    23.8
build                           9.27  15.8%   5095     6.5
research                        5.14   8.7%   2966     6.2
choose                          1.88   3.2%   1256     5.4
release                         1.35   2.3%    692     7.0
ship                            1.05   1.8%    545     6.9
bugs                            0.81   1.4%    494     5.9
log                             0.47   0.8%    349     4.8
done                            0.00   0.0%      2     3.8

slowest single steps
  pass    7    2558s  bg    shell                bg_16 completed: /tmp/g11/gate.sh > /tmp/g11/gate.out 2>&1; tail -20 /tmp/g11/gate/summary
  pass    7    2114s  model model                
  pass    7    1826s  tool  wait: gate           Waiting for the gate to finish
  pass    7    1818s  tool  wait: tests          Waiting for toast tests rebuild
  pass    8    1800s  tool  wait: gate           Waiting for the gate to finish
  pass   44    1800s  tool  wait: gate           Waiting for the gate
  pass   83    1800s  tool  wait: other          {}
  pass   21    1699s  tool  wait: gate           Waiting for merged-tree gate
  pass    7    1612s  tool  wait: tests          Waiting for test binary build
  pass    7    1611s  tool  wait: tests          Waiting for test binary link
  pass   10    1542s  tool  wait: gate           Waiting for the merged-tree gate
  pass    9    1481s  tool  wait: gate           Waiting for gate retry
  pass    7    1451s  model model                
  pass    4    1451s  tool  wait: gate           Waiting for gate step 1 retry
  pass   21    1449s  tool  wait: gate           Waiting for the gate
```

