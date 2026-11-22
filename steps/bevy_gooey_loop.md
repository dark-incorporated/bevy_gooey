# Where bevy_gooey_loop's time goes

Written by `tools/loop/steps.py publish` after every pass; the rows are in [`bevy_gooey_loop.tsv`](bevy_gooey_loop.tsv). Updated 2026-11-22 14:04Z.

## What the last passes shipped

From [`bevy_gooey_loop-audit.tsv`](bevy_gooey_loop-audit.tsv): commits on main, unpushed commits, package files changed, release tags, doc and demo files, and what was missing.

| pass | at | commits | unpushed | package_files | released | docs_files | demo_files | missing |
|---|---|---|---|---|---|---|---|---|
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
| 347 | 2026-11-22T14:04Z | 8 | 0 | 4 | v0.56.0 | 6 | 1 | - |

## How each configuration did

From [`bevy_gooey_loop-config.tsv`](bevy_gooey_loop-config.tsv): one row per pass with its host, wall time, model, thinking level and build settings.

```text
host       model                        think    jobs  threads  passes  mean min  median min
big        anthropic/claude-sonnet-5-5  high     8     8            11      41.7        36.4
big        anthropic/claude-sonnet-5-5  high     6     4            34      31.9        28.5
big        anthropic/claude-sonnet-5-5  high     3     4             4      37.9        35.7
```

## The last 12 passes

```text
12 pass(es), 2828 steps, 6.7 h of pass time

category                       hours  share  steps  mean s
wait: other                     2.85  42.3%    163    62.9
model                           2.01  29.9%   1336     5.4
cargo test                      0.47   6.9%     88    19.0
cargo build                     0.40   5.9%     70    20.4
shell                           0.25   3.8%    579     1.6
wait: build                     0.25   3.8%      2   454.9
phase mark                      0.13   1.9%     96     4.9
cargo publish                   0.09   1.4%     17    20.0
wait: gate                      0.05   0.8%      1   189.2
web                             0.03   0.5%     11    10.1
push gate                       0.03   0.4%     17     6.1
cargo clippy                    0.03   0.4%      7    14.5
wait: tests                     0.02   0.3%      1    78.1
github                          0.02   0.3%     37     2.1
clip record/render              0.02   0.3%      8     9.0

phase                          hours  share  steps  mean s
prove                           2.30  34.1%    815    10.1
build                           1.81  26.8%    878     7.4
gate                            1.41  20.9%    217    23.3
research                        0.40   6.0%    312     4.6
choose                          0.27   4.0%    234     4.1
release                         0.23   3.4%     92     8.8
ship                            0.12   1.8%     76     5.7
orient                          0.10   1.4%    102     3.4
bugs                            0.06   0.9%     40     5.2
log                             0.05   0.7%     44     4.1

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
67 pass(es), 18798 steps, 51.5 h of pass time

category                       hours  share  steps  mean s
model                          20.04  38.9%   8564     8.4
wait: gate                      9.91  19.3%     74   482.3
wait: other                     6.70  13.0%    289    83.5
wait: tests                     2.85   5.5%     34   302.0
cargo test                      2.83   5.5%    574    17.7
shell                           2.14   4.2%   3209     2.4
cargo build                     1.85   3.6%    381    17.5
wait: build                     1.16   2.3%     21   199.4
phase mark                      0.99   1.9%    487     7.3
wait: clip                      0.79   1.5%     30    94.9
remote wait                     0.33   0.6%     42    28.5
read files                      0.27   0.5%   2240     0.4
cargo publish                   0.26   0.5%     70    13.2
cargo clippy                    0.21   0.4%     82     9.1
github                          0.20   0.4%    203     3.6

phase                          hours  share  steps  mean s
orient                         19.08  37.1%   4565    15.1
prove                           9.59  18.6%   4567     7.6
gate                            8.53  16.6%   1304    23.6
build                           8.13  15.8%   4394     6.7
research                        1.96   3.8%   1285     5.5
choose                          1.52   2.9%    976     5.6
release                         1.15   2.2%    654     6.3
ship                            0.90   1.7%    525     6.2
log                             0.42   0.8%    325     4.7
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

