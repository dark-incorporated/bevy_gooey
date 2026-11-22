# Where bevy_gooey_loop's time goes

Written by `tools/loop/steps.py publish` after every pass; the rows are in [`bevy_gooey_loop.tsv`](bevy_gooey_loop.tsv). Updated 2026-11-22 15:29Z.

## What the last passes shipped

From [`bevy_gooey_loop-audit.tsv`](bevy_gooey_loop-audit.tsv): commits on main, unpushed commits, package files changed, release tags, doc and demo files, and what was missing.

| pass | at | commits | unpushed | package_files | released | docs_files | demo_files | missing |
|---|---|---|---|---|---|---|---|---|
| 89 | 2026-11-04T12:18Z | 6 | 0 | 4 | v0.34.0 | 2 | 0 | demo? |
| 90 | 2026-11-04T13:05Z | 7 | 0 | 8 | v0.35.0 | 7 | 1 | - |
| 91 | 2026-11-07T06:16Z | 11 | 0 | 7 | v0.38.0 | 8 | 2 | - |
| 341 | 2026-11-22T06:49Z | 0 | 1 | 0 | - | 0 | 0 | push |
| 342 | 2026-11-22T07:28Z | 8 | 0 | 4 | v0.52.2 | 9 | 2 | - |
| 343 | 2026-11-22T08:01Z | 0 | 0 | 0 | - | 0 | 0 | nothing shipped |
| 344 | 2026-11-22T08:44Z | 9 | 0 | 5 | - | 8 | 1 | release |
| 345 | 2026-11-22T09:40Z | 7 | 0 | 8 | v0.53.0,v0.54.0 | 6 | 1 | - |
| 346 | 2026-11-22T10:21Z | 7 | 0 | 6 | v0.55.0 | 8 | 1 | - |
| 347 | 2026-11-22T14:04Z | 8 | 0 | 4 | v0.56.0 | 6 | 1 | - |
| 348 | 2026-11-22T15:12Z | 7 | 0 | 6 | v0.57.0 | 6 | 1 | - |
| 349 | 2026-11-22T15:29Z | 0 | 0 | 0 | - | 0 | 0 | nothing shipped |

## How each configuration did

From [`bevy_gooey_loop-config.tsv`](bevy_gooey_loop-config.tsv): one row per pass with its host, wall time, model, thinking level and build settings.

```text
host       model                        think    jobs  threads  passes  mean min  median min
big        anthropic/claude-sonnet-5-5  high     8     8            11      41.7        36.4
big        anthropic/claude-sonnet-5-5  high     6     4            34      31.9        28.5
big        anthropic/claude-sonnet-5-5  high     3     4             6      39.3        35.7
```

## The last 12 passes

```text
12 pass(es), 2956 steps, 7.2 h of pass time

category                       hours  share  steps  mean s
wait: other                     3.42  47.7%    202    60.9
model                           1.92  26.8%   1373     5.0
cargo test                      0.44   6.2%     87    18.3
cargo build                     0.35   4.9%     70    18.0
shell                           0.27   3.8%    566     1.7
wait: build                     0.25   3.5%      2   454.9
phase mark                      0.11   1.5%     90     4.4
cargo publish                   0.08   1.1%     15    19.2
wait: gate                      0.05   0.7%      1   189.2
push gate                       0.04   0.6%     16     9.1
github                          0.04   0.5%     38     3.6
remote wait                     0.04   0.5%      8    16.4
web                             0.03   0.4%     11     9.1
cargo clippy                    0.02   0.3%      6    14.1
wait: tests                     0.02   0.3%      1    78.1

phase                          hours  share  steps  mean s
build                           2.32  32.3%   1121     7.4
prove                           1.96  27.4%    655    10.8
gate                            1.42  19.9%    218    23.5
research                        0.47   6.6%    325     5.2
release                         0.36   5.0%     98    13.2
choose                          0.31   4.3%    264     4.2
ship                            0.11   1.6%     74     5.6
orient                          0.10   1.4%    102     3.4
bugs                            0.06   0.8%     40     5.2
log                             0.05   0.7%     40     4.3

slowest single steps
  pass   90     910s  tool  wait: build          Waiting for example builds
  pass   90     417s  tool  wait: other          Waiting for mutation run
  pass  342     413s  bg    remote wait          bg_10 completed: while pgrep -f "cargo (nextest|test)" >/dev/null; do sleep 10; done; cat 
  pass   91     292s  tool  wait: other          {}
  pass   89     258s  tool  wait: other          {}
  pass   89     251s  tool  wait: other          {}
  pass  341     241s  tool  wait: other          {}
  pass  349     240s  tool  wait: other          {}
  pass  348     240s  tool  wait: other          {}
  pass  342     240s  tool  wait: other          {}
  pass  342     240s  tool  wait: other          {}
  pass  344     240s  tool  wait: other          {}
  pass  346     240s  tool  wait: other          {}
  pass  348     240s  tool  wait: other          {}
  pass  348     240s  tool  wait: other          {}
```

## Every pass recorded

```text
69 pass(es), 19338 steps, 52.9 h of pass time

category                       hours  share  steps  mean s
model                          20.31  38.4%   8801     8.3
wait: gate                      9.91  18.7%     74   482.3
wait: other                     7.66  14.5%    334    82.5
wait: tests                     2.85   5.4%     34   302.0
cargo test                      2.84   5.4%    582    17.6
shell                           2.17   4.1%   3288     2.4
cargo build                     1.91   3.6%    397    17.3
wait: build                     1.16   2.2%     21   199.4
phase mark                      1.00   1.9%    500     7.2
wait: clip                      0.79   1.5%     30    94.9
remote wait                     0.35   0.7%     45    28.0
read files                      0.28   0.5%   2310     0.4
cargo publish                   0.26   0.5%     71    13.0
github                          0.22   0.4%    208     3.8
cargo clippy                    0.21   0.4%     82     9.1

phase                          hours  share  steps  mean s
orient                         19.09  36.1%   4577    15.0
prove                           9.61  18.2%   4571     7.6
gate                            8.84  16.7%   1323    24.0
build                           8.78  16.6%   4720     6.7
research                        2.16   4.1%   1384     5.6
choose                          1.57   3.0%   1024     5.5
release                         1.32   2.5%    678     7.0
ship                            0.90   1.7%    527     6.2
log                             0.43   0.8%    329     4.7
bugs                            0.18   0.3%    111     5.9

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

