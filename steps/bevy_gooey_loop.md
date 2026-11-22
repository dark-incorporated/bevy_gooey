# Where bevy_gooey_loop's time goes

Written by `tools/loop/steps.py publish` after every pass; the rows are in [`bevy_gooey_loop.tsv`](bevy_gooey_loop.tsv). Updated 2026-11-22 10:21Z.

## What the last passes shipped

From [`bevy_gooey_loop-audit.tsv`](bevy_gooey_loop-audit.tsv): commits on main, unpushed commits, package files changed, release tags, doc and demo files, and what was missing.

| pass | at | commits | unpushed | package_files | released | docs_files | demo_files | missing |
|---|---|---|---|---|---|---|---|---|
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
| 345 | 2026-11-22T09:40Z | 7 | 0 | 8 | v0.53.0,v0.54.0 | 6 | 1 | - |
| 346 | 2026-11-22T10:21Z | 7 | 0 | 6 | v0.55.0 | 8 | 1 | - |

## How each configuration did

From [`bevy_gooey_loop-config.tsv`](bevy_gooey_loop-config.tsv): one row per pass with its host, wall time, model, thinking level and build settings.

```text
host       model                        think    jobs  threads  passes  mean min  median min
big        anthropic/claude-sonnet-5-5  high     8     8            11      41.7        36.4
big        anthropic/claude-sonnet-5-5  high     6     4            34      31.9        28.5
big        anthropic/claude-sonnet-5-5  high     3     4             3      42.4        40.6
```

## The last 12 passes

```text
12 pass(es), 2803 steps, 6.5 h of pass time

category                       hours  share  steps  mean s
wait: other                     2.73  41.9%    157    62.6
model                           2.03  31.2%   1323     5.5
cargo test                      0.45   6.9%     80    20.1
cargo build                     0.36   5.5%     65    19.8
wait: build                     0.25   3.9%      2   454.9
shell                           0.21   3.2%    572     1.3
phase mark                      0.13   2.0%     94     5.0
cargo publish                   0.09   1.4%     16    20.0
wait: gate                      0.05   0.8%      1   189.2
web                             0.03   0.5%     12     9.8
push gate                       0.03   0.5%     19     5.8
cargo clippy                    0.03   0.4%      8    12.7
wait: tests                     0.02   0.3%      1    78.1
github                          0.02   0.3%     35     2.2
clip record/render              0.02   0.3%      8     9.0

phase                          hours  share  steps  mean s
prove                           2.32  35.6%    825    10.1
build                           1.63  25.1%    840     7.0
gate                            1.33  20.5%    209    23.0
research                        0.41   6.3%    330     4.5
choose                          0.26   4.0%    230     4.1
release                         0.24   3.6%     96     8.9
orient                          0.10   1.6%    103     3.6
ship                            0.10   1.6%     70     5.3
bugs                            0.06   0.9%     40     5.2
log                             0.05   0.7%     42     4.1

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
  pass  346     240s  tool  wait: other          {}
  pass  342     240s  tool  wait: other          {}
  pass  345     240s  tool  wait: other          {}
```

## Every pass recorded

```text
66 pass(es), 18641 steps, 51.0 h of pass time

category                       hours  share  steps  mean s
model                          19.96  39.1%   8488     8.5
wait: gate                      9.91  19.4%     74   482.3
wait: other                     6.51  12.8%    280    83.7
wait: tests                     2.85   5.6%     34   302.0
cargo test                      2.80   5.5%    564    17.9
shell                           2.08   4.1%   3180     2.4
cargo build                     1.82   3.6%    376    17.4
wait: build                     1.16   2.3%     21   199.4
phase mark                      0.99   1.9%    478     7.4
wait: clip                      0.79   1.5%     30    94.9
remote wait                     0.33   0.7%     42    28.5
read files                      0.27   0.5%   2233     0.4
cargo publish                   0.24   0.5%     68    12.9
cargo clippy                    0.21   0.4%     82     9.1
github                          0.20   0.4%    199     3.7

phase                          hours  share  steps  mean s
orient                         19.08  37.4%   4554    15.1
prove                           9.58  18.8%   4559     7.6
gate                            8.40  16.5%   1290    23.4
build                           7.91  15.5%   4296     6.6
research                        1.95   3.8%   1278     5.5
choose                          1.51   3.0%    972     5.6
release                         1.14   2.2%    652     6.3
ship                            0.87   1.7%    515     6.1
log                             0.42   0.8%    323     4.7
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

