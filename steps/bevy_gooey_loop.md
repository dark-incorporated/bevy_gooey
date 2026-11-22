# Where bevy_gooey_loop's time goes

Written by `tools/loop/steps.py publish` after every pass; the rows are in [`bevy_gooey_loop.tsv`](bevy_gooey_loop.tsv). Updated 2026-11-22 08:44Z.

## What the last passes shipped

From [`bevy_gooey_loop-audit.tsv`](bevy_gooey_loop-audit.tsv): commits on main, unpushed commits, package files changed, release tags, doc and demo files, and what was missing.

| pass | at | commits | unpushed | package_files | released | docs_files | demo_files | missing |
|---|---|---|---|---|---|---|---|---|
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
| 343 | 2026-11-22T08:01Z | 0 | 0 | 0 | - | 0 | 0 | nothing shipped |
| 344 | 2026-11-22T08:44Z | 9 | 0 | 5 | - | 8 | 1 | release |

## How each configuration did

From [`bevy_gooey_loop-config.tsv`](bevy_gooey_loop-config.tsv): one row per pass with its host, wall time, model, thinking level and build settings.

```text
host       model                        think    jobs  threads  passes  mean min  median min
big        anthropic/claude-sonnet-5-5  high     8     8            11      41.7        36.4
big        anthropic/claude-sonnet-5-5  high     6     4            34      31.9        28.5
big        anthropic/claude-sonnet-5-5  high     3     4             1      30.9        30.9
```

## The last 12 passes

```text
12 pass(es), 2247 steps, 5.3 h of pass time

category                       hours  share  steps  mean s
wait: other                     2.24  42.4%    127    63.6
model                           1.68  31.8%   1057     5.7
cargo test                      0.38   7.2%     72    19.1
wait: build                     0.25   4.8%      2   454.9
cargo build                     0.17   3.2%     33    18.5
phase mark                      0.15   2.8%     89     6.0
shell                           0.12   2.4%    446     1.0
cargo publish                   0.07   1.3%     12    20.7
wait: gate                      0.05   1.0%      1   189.2
push gate                       0.03   0.6%     18     6.3
web                             0.03   0.5%      9    10.5
wait: tests                     0.02   0.4%      1    78.1
github                          0.02   0.4%     33     2.2
clip record/render              0.02   0.4%      8     9.0
cargo clippy                    0.02   0.3%      4    14.1

phase                          hours  share  steps  mean s
prove                           1.64  31.1%    611     9.7
build                           1.28  24.2%    596     7.7
gate                            1.23  23.2%    184    24.0
research                        0.43   8.2%    311     5.0
choose                          0.22   4.1%    206     3.8
release                         0.21   3.9%     85     8.8
orient                          0.10   2.0%    103     3.6
ship                            0.08   1.5%     62     4.5
bugs                            0.06   1.1%     44     4.9
log                             0.04   0.7%     32     4.1

slowest single steps
  pass   90     910s  tool  wait: build          Waiting for example builds
  pass   87     732s  tool  wait: other          {}
  pass   90     417s  tool  wait: other          Waiting for mutation run
  pass  342     413s  bg    remote wait          bg_10 completed: while pgrep -f "cargo (nextest|test)" >/dev/null; do sleep 10; done; cat 
  pass   88     298s  tool  wait: other          {}
  pass   91     292s  tool  wait: other          {}
  pass   89     258s  tool  wait: other          {}
  pass   89     251s  tool  wait: other          {}
  pass  341     241s  tool  wait: other          {}
  pass  342     240s  tool  wait: other          {}
  pass  342     240s  tool  wait: other          {}
  pass  344     240s  tool  wait: other          {}
  pass  342     240s  tool  wait: other          {}
  pass   91     237s  bg    shell                bg_1 completed: cat >> tests/serde.rs <<'E' #[test] fn a_skins_motion_palette_is_data_and_
  pass   88     222s  tool  wait: other          {}
```

## Every pass recorded

```text
64 pass(es), 17879 steps, 49.4 h of pass time

category                       hours  share  steps  mean s
model                          19.46  39.4%   8130     8.6
wait: gate                      9.91  20.0%     74   482.3
wait: other                     5.94  12.0%    248    86.3
wait: tests                     2.85   5.8%     34   302.0
cargo test                      2.65   5.4%    545    17.5
shell                           1.99   4.0%   3019     2.4
cargo build                     1.63   3.3%    344    17.0
wait: build                     1.16   2.4%     21   199.4
phase mark                      0.96   1.9%    459     7.5
wait: clip                      0.79   1.6%     30    94.9
remote wait                     0.31   0.6%     39    29.0
read files                      0.26   0.5%   2156     0.4
cargo publish                   0.22   0.4%     62    12.5
github                          0.20   0.4%    193     3.8
cargo clippy                    0.19   0.4%     77     8.9

phase                          hours  share  steps  mean s
orient                         19.07  38.6%   4543    15.1
prove                           8.91  18.0%   4345     7.4
gate                            8.17  16.5%   1256    23.4
build                           7.46  15.1%   3977     6.8
research                        1.86   3.8%   1194     5.6
choose                          1.45   2.9%    927     5.6
release                         1.09   2.2%    624     6.3
ship                            0.85   1.7%    507     6.0
log                             0.41   0.8%    309     4.7
bugs                            0.18   0.4%    111     5.9

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

