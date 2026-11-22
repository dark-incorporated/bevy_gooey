# Where bevy_gooey_loop's time goes

Written by `tools/loop/steps.py publish` after every pass; the rows are in [`bevy_gooey_loop.tsv`](bevy_gooey_loop.tsv). Updated 2026-11-22 07:28Z.

## What the last passes shipped

From [`bevy_gooey_loop-audit.tsv`](bevy_gooey_loop-audit.tsv): commits on main, unpushed commits, package files changed, release tags, doc and demo files, and what was missing.

| pass | at | commits | unpushed | package_files | released | docs_files | demo_files | missing |
|---|---|---|---|---|---|---|---|---|
| 82 | 2026-11-02T17:53Z | 6 | 0 | 5 | v0.31.3 | 6 | 1 | - |
| 83 | 2026-11-02T18:55Z | 7 | 0 | 5 | v0.32.0 | 6 | 1 | - |
| 84 | 2026-11-03T11:52Z | 5 | 0 | 2 | v0.32.1 | 2 | 0 | demo? |
| 85 | 2026-11-03T12:05Z | 7 | 0 | 5 | v0.32.2 | 4 | 0 | demo? |
| 86 | 2026-11-03T12:18Z | 7 | 0 | 3 | v0.32.3 | 4 | 0 | demo? |
| 87 | 2026-11-03T12:45Z | 8 | 0 | 4 | v0.32.4 | 7 | 1 | - |
| 88 | 2026-11-03T13:17Z | 9 | 0 | 6 | v0.33.0 | 7 | 1 | - |
| 89 | 2026-11-04T12:18Z | 6 | 0 | 4 | v0.34.0 | 2 | 0 | demo? |
| 90 | 2026-11-04T13:05Z | 7 | 0 | 8 | v0.35.0 | 7 | 1 | - |
| 91 | 2026-11-07T06:16Z | 11 | 0 | 7 | v0.38.0 | 8 | 2 | - |
| 341 | 2026-11-22T06:49Z | 0 | 1 | 0 | - | 0 | 0 | push |
| 342 | 2026-11-22T07:28Z | 8 | 0 | 4 | v0.52.2 | 9 | 2 | - |

## How each configuration did

From [`bevy_gooey_loop-config.tsv`](bevy_gooey_loop-config.tsv): one row per pass with its host, wall time, model, thinking level and build settings.

```text
host       model                        think    jobs  threads  passes  mean min  median min
big        anthropic/claude-sonnet-5-5  high     8     8            11      41.7        36.4
big        anthropic/claude-sonnet-5-5  high     6     4            33      32.5        29.7
```

## The last 12 passes

```text
12 pass(es), 2116 steps, 5.9 h of pass time

category                       hours  share  steps  mean s
wait: other                     2.70  46.0%    114    85.3
model                           1.69  28.8%    997     6.1
cargo test                      0.35   6.0%     60    21.2
wait: build                     0.25   4.3%      2   454.9
cargo build                     0.19   3.3%     36    19.1
phase mark                      0.18   3.0%     92     6.9
shell                           0.14   2.3%    408     1.2
wait: gate                      0.10   1.6%      2   172.1
cargo publish                   0.08   1.4%     13    22.3
push gate                       0.04   0.6%     22     6.1
web                             0.03   0.4%      9    10.4
clip record/render              0.02   0.4%     12     7.4
cargo clippy                    0.02   0.4%      6    13.2
wait: tests                     0.02   0.4%      1    78.1
remote wait                     0.02   0.3%      3    21.8

phase                          hours  share  steps  mean s
gate                            1.93  32.8%    201    34.5
prove                           1.50  25.6%    580     9.3
build                           1.33  22.6%    582     8.2
research                        0.42   7.2%    257     5.9
release                         0.24   4.1%     97     8.9
choose                          0.17   3.0%    158     3.9
orient                          0.10   1.8%    106     3.5
ship                            0.08   1.3%     58     4.8
bugs                            0.06   1.0%     39     5.2
log                             0.04   0.6%     30     4.2

slowest single steps
  pass   83    1800s  tool  wait: other          {}
  pass   90     910s  tool  wait: build          Waiting for example builds
  pass   87     732s  tool  wait: other          {}
  pass   83     537s  tool  wait: other          {}
  pass   90     417s  tool  wait: other          Waiting for mutation run
  pass  342     413s  bg    remote wait          bg_10 completed: while pgrep -f "cargo (nextest|test)" >/dev/null; do sleep 10; done; cat 
  pass   88     298s  tool  wait: other          {}
  pass   91     292s  tool  wait: other          {}
  pass   89     258s  tool  wait: other          {}
  pass   89     251s  tool  wait: other          {}
  pass  341     241s  tool  wait: other          {}
  pass  342     240s  tool  wait: other          {}
  pass  342     240s  tool  wait: other          {}
  pass  342     240s  tool  wait: other          {}
  pass   91     237s  bg    shell                bg_1 completed: cat >> tests/serde.rs <<'E' #[test] fn a_skins_motion_palette_is_data_and_
```

## Every pass recorded

```text
62 pass(es), 17426 steps, 48.8 h of pass time

category                       hours  share  steps  mean s
model                          19.18  39.3%   7917     8.7
wait: gate                      9.91  20.3%     74   482.3
wait: other                     5.69  11.7%    231    88.6
wait: tests                     2.85   5.9%     34   302.0
cargo test                      2.58   5.3%    527    17.6
shell                           1.97   4.0%   2910     2.4
cargo build                     1.61   3.3%    338    17.2
wait: build                     1.16   2.4%     21   199.4
phase mark                      0.96   2.0%    447     7.7
wait: clip                      0.79   1.6%     30    94.9
remote wait                     0.31   0.6%     39    29.0
read files                      0.26   0.5%   2106     0.4
cargo publish                   0.22   0.4%     61    12.7
github                          0.20   0.4%    186     3.8
cargo clippy                    0.19   0.4%     77     8.9

phase                          hours  share  steps  mean s
orient                         19.07  39.1%   4534    15.1
prove                           8.61  17.6%   4186     7.4
gate                            8.05  16.5%   1240    23.4
build                           7.39  15.2%   3908     6.8
research                        1.76   3.6%   1092     5.8
choose                          1.39   2.9%    857     5.8
release                         1.08   2.2%    616     6.3
ship                            0.84   1.7%    501     6.1
log                             0.40   0.8%    305     4.7
bugs                            0.18   0.4%    106     6.0

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

