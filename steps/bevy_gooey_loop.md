# Where bevy_gooey_loop's time goes

Written by `tools/loop/steps.py publish` after every pass; the rows are in [`bevy_gooey_loop.tsv`](bevy_gooey_loop.tsv). Updated 2026-11-27 16:54Z.

## What the last passes shipped

From [`bevy_gooey_loop-audit.tsv`](bevy_gooey_loop-audit.tsv): commits on main, unpushed commits, package files changed, release tags, doc and demo files, and what was missing.

| pass | at | commits | unpushed | package_files | released | docs_files | demo_files | missing |
|---|---|---|---|---|---|---|---|---|
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
| 367 | 2026-11-27T16:51Z | 0 | 0 | 0 | - | 0 | 0 | nothing shipped |
| 368 | 2026-11-27T16:54Z | 0 | 0 | 0 | - | 0 | 0 | nothing shipped |

## How each configuration did

From [`bevy_gooey_loop-config.tsv`](bevy_gooey_loop-config.tsv): one row per pass with its host, wall time, model, thinking level and build settings.

```text
host       model                        think    jobs  threads  passes  mean min  median min
big        anthropic/claude-sonnet-5-5  high     8     8            11      41.7        36.4
big        anthropic/claude-sonnet-5-5  high     6     4            34      31.9        28.5
big        anthropic/claude-sonnet-5-5  high     3     4             8      37.0        33.9
big        ollama-cloud/deepseek-v4.1-flash:1017 high     3     4            17      28.5         7.5
```

## The last 12 passes

```text
12 pass(es), 1724 steps, 3.2 h of pass time

category                       hours  share  steps  mean s
wait: other                     1.18  36.6%     75    56.5
model                           0.85  26.3%    807     3.8
cargo test                      0.49  15.2%     70    25.2
cargo build                     0.32  10.0%     24    48.2
shell                           0.15   4.6%    547     1.0
homestill                       0.06   1.9%     38     5.9
remote wait                     0.05   1.6%      7    25.8
cargo publish                   0.05   1.5%      7    24.7
cargo clippy                    0.03   0.8%      2    47.0
phase mark                      0.02   0.7%     27     2.8
web                             0.01   0.2%      3     9.1
github                          0.01   0.2%     22     1.1
git                             0.01   0.2%     28     0.7
push gate                       0.00   0.1%      6     2.1
clip record/render              0.00   0.1%      4     3.1

phase                          hours  share  steps  mean s
build                           0.90  28.1%    402     8.1
log                             0.78  24.3%    315     8.9
gate                            0.42  13.0%     92    16.4
research                        0.41  12.7%    366     4.0
prove                           0.36  11.3%    176     7.4
orient                          0.18   5.7%    259     2.6
bugs                            0.06   1.9%     17    13.0
choose                          0.05   1.7%     60     3.2
release                         0.04   1.4%     30     5.3
ship                            0.00   0.0%      4     1.0

slowest single steps
  pass  359     300s  bg    cargo test           bg_1 failed: RUSTC_WRAPPER= CARGO_TARGET_DIR=$HOME/.cargo-target-shared/global cargo nexte
  pass  359     241s  tool  wait: other          {}
  pass  359     240s  tool  wait: other          {}
  pass  359     240s  tool  wait: other          {}
  pass  357     240s  tool  wait: other          {}
  pass  358     240s  tool  wait: other          {"timeout": 2400}
  pass  359     220s  tool  wait: other          {}
  pass  359     210s  tool  wait: other          {}
  pass  358     203s  tool  wait: other          {"timeout": 2400}
  pass  359     182s  tool  wait: other          {}
  pass  359     181s  tool  wait: other          {}
  pass  357     170s  tool  wait: other          {}
  pass  359     170s  tool  wait: other          {}
  pass  359     161s  tool  wait: other          {}
  pass  359     158s  tool  wait: other          {}
```

## Every pass recorded

```text
88 pass(es), 24378 steps, 62.0 h of pass time

category                       hours  share  steps  mean s
model                          23.09  37.3%  11132     7.5
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
orient                         19.67  31.7%   5235    13.5
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

