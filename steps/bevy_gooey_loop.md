# Where bevy_gooey_loop's time goes

Written by `tools/loop/steps.py publish` after every pass; the rows are in [`bevy_gooey_loop.tsv`](bevy_gooey_loop.tsv). Updated 2026-11-27 16:07Z.

## What the last passes shipped

From [`bevy_gooey_loop-audit.tsv`](bevy_gooey_loop-audit.tsv): commits on main, unpushed commits, package files changed, release tags, doc and demo files, and what was missing.

| pass | at | commits | unpushed | package_files | released | docs_files | demo_files | missing |
|---|---|---|---|---|---|---|---|---|
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
| 358 | 2026-11-27T14:27Z | 4 | 0 | 4 | v0.63.0 | 4 | 0 | demo? |
| 359 | 2026-11-27T16:07Z | 3 | 0 | 4 | v0.64.0 | 6 | 1 | - |

## How each configuration did

From [`bevy_gooey_loop-config.tsv`](bevy_gooey_loop-config.tsv): one row per pass with its host, wall time, model, thinking level and build settings.

```text
host       model                        think    jobs  threads  passes  mean min  median min
big        anthropic/claude-sonnet-5-5  high     8     8            11      41.7        36.4
big        anthropic/claude-sonnet-5-5  high     6     4            34      31.9        28.5
big        anthropic/claude-sonnet-5-5  high     3     4             8      37.0        33.9
big        ollama-cloud/deepseek-v4.1-flash:1017 high     3     4             8      59.3        54.7
```

## The last 12 passes

```text
12 pass(es), 5483 steps, 10.3 h of pass time

category                       hours  share  steps  mean s
wait: other                     4.24  41.1%    272    56.2
model                           2.91  28.1%   2528     4.1
cargo test                      1.47  14.3%    187    28.4
cargo build                     0.63   6.1%    111    20.4
shell                           0.50   4.8%   1632     1.1
cargo publish                   0.14   1.4%     20    25.4
phase mark                      0.08   0.8%     80     3.5
homestill                       0.08   0.8%     62     4.5
remote wait                     0.07   0.7%     12    20.2
cargo clippy                    0.05   0.5%     13    14.5
github                          0.04   0.4%     68     2.3
git                             0.03   0.3%    129     1.0
push gate                       0.03   0.3%     24     4.4
web                             0.02   0.2%     10     8.3
read files                      0.02   0.2%    195     0.3

phase                          hours  share  steps  mean s
research                        3.53  34.1%   2001     6.3
build                           2.04  19.7%   1103     6.6
gate                            1.27  12.3%    206    22.2
log                             0.83   8.0%    339     8.8
prove                           0.69   6.7%    354     7.0
bugs                            0.63   6.1%    383     6.0
orient                          0.53   5.2%    636     3.0
choose                          0.42   4.0%    340     4.4
release                         0.25   2.4%     68    13.1
ship                            0.15   1.5%     24    22.9
done                            0.00   0.0%      2     3.8

slowest single steps
  pass  356     904s  bg    cargo build          bg_41 completed: CARGO_INCREMENTAL=0 RUSTC_WRAPPER= CARGO_BUILD_JOBS=2 timeout 900 cargo b
  pass  353     735s  bg    shell                bg_18 completed: cp /tmp/g117-fix.rs src/code_overlay.rs && export CARGO_TARGET_DIR=$HOME/
  pass  356     594s  model model                
  pass  359     300s  bg    cargo test           bg_1 failed: RUSTC_WRAPPER= CARGO_TARGET_DIR=$HOME/.cargo-target-shared/global cargo nexte
  pass  353     300s  bg    shell                bg_8 failed: export CARGO_TARGET_DIR=$HOME/.cargo-target-shared/global && unset CARGO_INCR
  pass  356     300s  bg    cargo build          bg_20 failed: CARGO_INCREMENTAL=0 RUSTC_WRAPPER= cargo build --release --lib 2>&1 | tail -
  pass  356     247s  tool  wait: other          {}
  pass  352     242s  bg    cargo test           bg_9 completed: RUSTC_WRAPPER="" CARGO_BUILD_RUSTC_WRAPPER="" CARGO_INCREMENTAL=0 CARGO_BU
  pass  353     242s  bg    shell                bg_11 completed: export CARGO_TARGET_DIR=$HOME/.cargo-target-shared/global && unset CARGO_
  pass  359     241s  tool  wait: other          {}
  pass  356     240s  tool  wait: other          {}
  pass  349     240s  tool  wait: other          {}
  pass  352     240s  tool  wait: other          {}
  pass  356     240s  bg    cargo build          bg_23 completed: CARGO_INCREMENTAL=0 RUSTC_WRAPPER= timeout 240 cargo build --release --li
  pass  348     240s  tool  wait: other          {}
```

## Every pass recorded

```text
79 pass(es), 24281 steps, 61.8 h of pass time

category                       hours  share  steps  mean s
model                          22.95  37.1%  11092     7.4
wait: other                    10.94  17.7%    561    70.2
wait: gate                      9.91  16.0%     74   482.3
cargo test                      4.30   7.0%    761    20.3
wait: tests                     2.85   4.6%     34   302.0
shell                           2.64   4.3%   4841     2.0
cargo build                     2.48   4.0%    492    18.2
wait: build                     1.16   1.9%     21   199.4
phase mark                      1.07   1.7%    567     6.8
wait: clip                      0.79   1.3%     30    94.9
remote wait                     0.40   0.6%     54    26.6
cargo publish                   0.40   0.6%     90    15.9
read files                      0.29   0.5%   2435     0.4
cargo clippy                    0.26   0.4%     95     9.9
github                          0.25   0.4%    271     3.3

phase                          hours  share  steps  mean s
orient                         19.62  31.7%   5201    13.6
prove                          10.28  16.6%   4921     7.5
build                          10.17  16.5%   5497     6.7
gate                            9.80  15.9%   1510    23.4
research                        5.49   8.9%   3286     6.0
choose                          1.93   3.1%   1316     5.3
release                         1.39   2.3%    722     7.0
log                             1.25   2.0%    664     6.8
ship                            1.05   1.7%    549     6.9
bugs                            0.81   1.3%    494     5.9
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

