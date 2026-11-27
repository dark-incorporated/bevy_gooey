# Where bevy_gooey_loop's time goes

Written by `tools/loop/steps.py publish` after every pass; the rows are in [`bevy_gooey_loop.tsv`](bevy_gooey_loop.tsv). Updated 2026-11-27 16:49Z.

## What the last passes shipped

From [`bevy_gooey_loop-audit.tsv`](bevy_gooey_loop-audit.tsv): commits on main, unpushed commits, package files changed, release tags, doc and demo files, and what was missing.

| pass | at | commits | unpushed | package_files | released | docs_files | demo_files | missing |
|---|---|---|---|---|---|---|---|---|
| 355 | 2026-11-26T01:05Z | 5 | 0 | 3 | v0.60.0 | 10 | 2 | - |
| 356 | 2026-11-26T03:00Z | 15 | 0 | 4 | v0.61.0 | 6 | 1 | - |
| 357 | 2026-11-27T13:37Z | 6 | 0 | 3 | v0.62.0 | 7 | 1 | - |
| 358 | 2026-11-27T14:27Z | 4 | 0 | 4 | v0.63.0 | 4 | 0 | demo? |
| 359 | 2026-11-27T16:07Z | 3 | 0 | 4 | v0.64.0 | 6 | 1 | - |
| 360 | 2026-11-27T16:41Z | 0 | 0 | 0 | - | 0 | 0 | nothing shipped |
| 361 | 2026-11-27T16:42Z | 0 | 0 | 0 | - | 0 | 0 | nothing shipped |
| 362 | 2026-11-27T16:43Z | 0 | 0 | 0 | - | 0 | 0 | nothing shipped |
| 363 | 2026-11-27T16:43Z | 0 | 0 | 0 | - | 0 | 0 | nothing shipped |
| 364 | 2026-11-27T16:44Z | 0 | 0 | 0 | - | 0 | 0 | nothing shipped |
| 365 | 2026-11-27T16:46Z | 0 | 0 | 0 | - | 0 | 0 | nothing shipped |
| 366 | 2026-11-27T16:49Z | 0 | 0 | 0 | - | 0 | 0 | nothing shipped |

## How each configuration did

From [`bevy_gooey_loop-config.tsv`](bevy_gooey_loop-config.tsv): one row per pass with its host, wall time, model, thinking level and build settings.

```text
host       model                        think    jobs  threads  passes  mean min  median min
big        anthropic/claude-sonnet-5-5  high     8     8            11      41.7        36.4
big        anthropic/claude-sonnet-5-5  high     6     4            34      31.9        28.5
big        anthropic/claude-sonnet-5-5  high     3     4             8      37.0        33.9
big        ollama-cloud/deepseek-v4.1-flash:1017 high     3     4            15      32.3        17.0
```

## The last 12 passes

```text
12 pass(es), 3365 steps, 6.2 h of pass time

category                       hours  share  steps  mean s
wait: other                     1.90  30.8%    143    47.8
model                           1.85  30.0%   1561     4.3
cargo test                      1.18  19.2%    128    33.3
cargo build                     0.50   8.1%     72    24.9
shell                           0.35   5.7%   1079     1.2
cargo publish                   0.10   1.6%     13    27.8
homestill                       0.08   1.2%     45     6.0
remote wait                     0.05   0.8%      7    25.8
phase mark                      0.05   0.8%     44     3.8
cargo clippy                    0.04   0.6%      5    26.3
github                          0.02   0.3%     43     1.7
web                             0.01   0.2%      6     9.0
sleep                           0.01   0.2%      1    45.4
read files                      0.01   0.2%     84     0.4
push gate                       0.01   0.2%     17     2.1

phase                          hours  share  steps  mean s
research                        2.32  37.6%   1253     6.7
build                           1.23  20.0%    672     6.6
log                             0.80  13.0%    325     8.9
gate                            0.63  10.3%    144    15.8
prove                           0.37   6.0%    192     6.9
choose                          0.34   5.6%    278     4.5
orient                          0.33   5.3%    426     2.8
bugs                            0.06   1.0%     17    13.0
release                         0.04   0.7%     30     5.3
ship                            0.04   0.6%     12    10.6

slowest single steps
  pass  356     904s  bg    cargo build          bg_41 completed: CARGO_INCREMENTAL=0 RUSTC_WRAPPER= CARGO_BUILD_JOBS=2 timeout 900 cargo b
  pass  356     594s  model model                
  pass  359     300s  bg    cargo test           bg_1 failed: RUSTC_WRAPPER= CARGO_TARGET_DIR=$HOME/.cargo-target-shared/global cargo nexte
  pass  356     300s  bg    cargo build          bg_20 failed: CARGO_INCREMENTAL=0 RUSTC_WRAPPER= cargo build --release --lib 2>&1 | tail -
  pass  356     247s  tool  wait: other          {}
  pass  359     241s  tool  wait: other          {}
  pass  356     240s  tool  wait: other          {}
  pass  356     240s  bg    cargo build          bg_23 completed: CARGO_INCREMENTAL=0 RUSTC_WRAPPER= timeout 240 cargo build --release --li
  pass  355     240s  tool  wait: other          {}
  pass  359     240s  tool  wait: other          {}
  pass  359     240s  tool  wait: other          {}
  pass  357     240s  tool  wait: other          {}
  pass  358     240s  tool  wait: other          {"timeout": 2400}
  pass  356     232s  tool  wait: other          {}
  pass  355     231s  tool  wait: other          {}
```

## Every pass recorded

```text
86 pass(es), 24376 steps, 62.0 h of pass time

category                       hours  share  steps  mean s
model                          23.09  37.3%  11130     7.5
wait: other                    10.94  17.7%    561    70.2
wait: gate                      9.91  16.0%     74   482.3
cargo test                      4.30   6.9%    761    20.3
wait: tests                     2.85   4.6%     34   302.0
shell                           2.64   4.3%   4882     1.9
cargo build                     2.48   4.0%    492    18.2
wait: build                     1.16   1.9%     21   199.4
phase mark                      1.08   1.7%    570     6.8
wait: clip                      0.79   1.3%     30    94.9
remote wait                     0.40   0.6%     54    26.6
cargo publish                   0.40   0.6%     90    15.9
read files                      0.29   0.5%   2436     0.4
cargo clippy                    0.26   0.4%     95     9.9
github                          0.25   0.4%    273     3.3

phase                          hours  share  steps  mean s
orient                         19.66  31.7%   5233    13.5
prove                          10.28  16.6%   4921     7.5
build                          10.17  16.4%   5497     6.7
gate                            9.80  15.8%   1510    23.4
research                        5.55   9.0%   3332     6.0
choose                          1.93   3.1%   1316     5.3
release                         1.39   2.3%    722     7.0
log                             1.25   2.0%    664     6.8
ship                            1.05   1.7%    549     6.9
bugs                            0.88   1.4%    511     6.2
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

