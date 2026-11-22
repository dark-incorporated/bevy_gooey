# Where bevy_gooey_loop's time goes

Written by `tools/loop/steps.py publish` after every pass; the rows are in [`bevy_gooey_loop.tsv`](bevy_gooey_loop.tsv). Updated 2026-11-22 09:40Z.

## What the last passes shipped

From [`bevy_gooey_loop-audit.tsv`](bevy_gooey_loop-audit.tsv): commits on main, unpushed commits, package files changed, release tags, doc and demo files, and what was missing.

| pass | at | commits | unpushed | package_files | released | docs_files | demo_files | missing |
|---|---|---|---|---|---|---|---|---|
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
| 345 | 2026-11-22T09:40Z | 7 | 0 | 8 | v0.53.0,v0.54.0 | 6 | 1 | - |

## How each configuration did

From [`bevy_gooey_loop-config.tsv`](bevy_gooey_loop-config.tsv): one row per pass with its host, wall time, model, thinking level and build settings.

```text
host       model                        think    jobs  threads  passes  mean min  median min
big        anthropic/claude-sonnet-5-5  high     8     8            11      41.7        36.4
big        anthropic/claude-sonnet-5-5  high     6     4            34      31.9        28.5
big        anthropic/claude-sonnet-5-5  high     3     4             2      43.3        43.3
```

## The last 12 passes

```text
12 pass(es), 2604 steps, 6.1 h of pass time

category                       hours  share  steps  mean s
wait: other                     2.60  43.0%    141    66.4
model                           1.92  31.7%   1219     5.7
cargo test                      0.42   6.9%     78    19.2
wait: build                     0.25   4.2%      2   454.9
cargo build                     0.24   4.0%     52    16.8
shell                           0.16   2.6%    508     1.1
phase mark                      0.14   2.4%     92     5.7
cargo publish                   0.08   1.4%     17    18.0
wait: gate                      0.05   0.9%      1   189.2
push gate                       0.03   0.5%     19     5.9
web                             0.03   0.5%     11     9.5
cargo clippy                    0.03   0.4%      7    13.2
wait: tests                     0.02   0.4%      1    78.1
github                          0.02   0.3%     32     2.3
clip record/render              0.02   0.3%      8     9.0

phase                          hours  share  steps  mean s
prove                           2.09  34.5%    721    10.4
build                           1.50  24.8%    792     6.8
gate                            1.27  21.0%    188    24.4
research                        0.38   6.3%    299     4.6
choose                          0.26   4.3%    235     4.0
release                         0.26   4.2%    107     8.6
orient                          0.11   1.7%    103     3.7
ship                            0.08   1.4%     64     4.7
bugs                            0.06   1.0%     40     5.2
log                             0.04   0.7%     40     3.9

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
  pass  345     240s  tool  wait: other          {}
  pass  345     240s  tool  wait: other          {}
```

## Every pass recorded

```text
65 pass(es), 18302 steps, 50.4 h of pass time

category                       hours  share  steps  mean s
model                          19.76  39.2%   8325     8.5
wait: gate                      9.91  19.7%     74   482.3
wait: other                     6.35  12.6%    263    86.9
wait: tests                     2.85   5.7%     34   302.0
cargo test                      2.72   5.4%    556    17.6
shell                           2.03   4.0%   3091     2.4
cargo build                     1.70   3.4%    363    16.9
wait: build                     1.16   2.3%     21   199.4
phase mark                      0.98   1.9%    469     7.5
wait: clip                      0.79   1.6%     30    94.9
remote wait                     0.31   0.6%     40    28.3
read files                      0.26   0.5%   2219     0.4
cargo publish                   0.23   0.5%     67    12.4
github                          0.20   0.4%    194     3.7
cargo clippy                    0.20   0.4%     80     9.0

phase                          hours  share  steps  mean s
orient                         19.08  37.9%   4548    15.1
prove                           9.35  18.6%   4455     7.6
gate                            8.28  16.4%   1264    23.6
build                           7.69  15.3%   4177     6.6
research                        1.89   3.8%   1223     5.6
choose                          1.49   3.0%    956     5.6
release                         1.14   2.3%    652     6.3
ship                            0.85   1.7%    509     6.0
log                             0.41   0.8%    319     4.7
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

