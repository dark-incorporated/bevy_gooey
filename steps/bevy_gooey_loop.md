# Where bevy_gooey_loop's time goes

Written by `tools/loop/steps.py publish` after every pass; the rows are in [`bevy_gooey_loop.tsv`](bevy_gooey_loop.tsv). Updated 2026-11-27 13:37Z.

## What the last passes shipped

From [`bevy_gooey_loop-audit.tsv`](bevy_gooey_loop-audit.tsv): commits on main, unpushed commits, package files changed, release tags, doc and demo files, and what was missing.

| pass | at | commits | unpushed | package_files | released | docs_files | demo_files | missing |
|---|---|---|---|---|---|---|---|---|
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
| 357 | 2026-11-27T13:37Z | 6 | 0 | 3 | v0.62.0 | 7 | 1 | - |

## How each configuration did

From [`bevy_gooey_loop-config.tsv`](bevy_gooey_loop-config.tsv): one row per pass with its host, wall time, model, thinking level and build settings.

```text
host       model                        think    jobs  threads  passes  mean min  median min
big        anthropic/claude-sonnet-5-5  high     8     8            11      41.7        36.4
big        anthropic/claude-sonnet-5-5  high     6     4            34      31.9        28.5
big        anthropic/claude-sonnet-5-5  high     3     4             8      37.0        33.9
big        ollama-cloud/deepseek-v4.1-flash:1017 high     3     4             6      54.1        48.0
```

## The last 12 passes

```text
12 pass(es), 4838 steps, 8.9 h of pass time

category                       hours  share  steps  mean s
wait: other                     3.62  40.5%    234    55.7
model                           2.68  29.9%   2214     4.4
cargo test                      1.17  13.1%    146    28.9
shell                           0.52   5.8%   1421     1.3
cargo build                     0.48   5.3%    107    16.1
cargo publish                   0.14   1.6%     19    26.6
phase mark                      0.07   0.8%     80     3.3
github                          0.04   0.5%     66     2.4
git                             0.04   0.4%    127     1.1
remote wait                     0.04   0.4%      7    18.1
cargo clippy                    0.03   0.4%     13     9.5
push gate                       0.03   0.3%     22     4.6
homestill                       0.03   0.3%     48     2.1
read files                      0.02   0.2%    185     0.4
web                             0.02   0.2%      9     8.5

phase                          hours  share  steps  mean s
research                        3.40  38.0%   1927     6.4
build                           1.58  17.6%    918     6.2
gate                            1.10  12.3%    154    25.8
prove                           0.87   9.7%    424     7.4
bugs                            0.63   7.1%    383     6.0
orient                          0.48   5.3%    560     3.1
choose                          0.43   4.8%    336     4.6
release                         0.21   2.3%     40    18.6
ship                            0.20   2.2%     36    19.9
log                             0.05   0.6%     30     6.2
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
77 pass(es), 23140 steps, 59.3 h of pass time

category                       hours  share  steps  mean s
model                          22.43  37.8%  10539     7.7
wait: other                     9.97  16.8%    497    72.2
wait: gate                      9.91  16.7%     74   482.3
cargo test                      3.90   6.6%    702    20.0
wait: tests                     2.85   4.8%     34   302.0
shell                           2.55   4.3%   4512     2.0
cargo build                     2.18   3.7%    470    16.7
wait: build                     1.16   2.0%     21   199.4
phase mark                      1.05   1.8%    549     6.9
wait: clip                      0.79   1.3%     30    94.9
cargo publish                   0.37   0.6%     86    15.6
remote wait                     0.35   0.6%     47    26.8
read files                      0.29   0.5%   2404     0.4
github                          0.25   0.4%    260     3.4
cargo clippy                    0.23   0.4%     93     9.1

phase                          hours  share  steps  mean s
orient                         19.55  33.0%   5108    13.8
prove                          10.22  17.2%   4879     7.5
gate                            9.38  15.8%   1418    23.8
build                           9.27  15.6%   5095     6.5
research                        5.29   8.9%   3150     6.0
choose                          1.92   3.2%   1292     5.4
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

