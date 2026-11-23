# Where bevy_gooey_loop's time goes

Written by `tools/loop/steps.py publish` after every pass; the rows are in [`bevy_gooey_loop.tsv`](bevy_gooey_loop.tsv). Updated 2026-11-23 00:30Z.

## What the last passes shipped

From [`bevy_gooey_loop-audit.tsv`](bevy_gooey_loop-audit.tsv): commits on main, unpushed commits, package files changed, release tags, doc and demo files, and what was missing.

| pass | at | commits | unpushed | package_files | released | docs_files | demo_files | missing |
|---|---|---|---|---|---|---|---|---|
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
| 350 | 2026-11-23T00:06Z | 5 | 0 | 5 | v0.58.0 | 6 | 1 | - |
| 351 | 2026-11-23T00:30Z | 4 | 0 | 6 | - | 8 | 1 | release |

## How each configuration did

From [`bevy_gooey_loop-config.tsv`](bevy_gooey_loop-config.tsv): one row per pass with its host, wall time, model, thinking level and build settings.

```text
host       model                        think    jobs  threads  passes  mean min  median min
big        anthropic/claude-sonnet-5-5  high     8     8            11      41.7        36.4
big        anthropic/claude-sonnet-5-5  high     6     4            34      31.9        28.5
big        anthropic/claude-sonnet-5-5  high     3     4             8      37.0        33.9
```

## The last 12 passes

```text
12 pass(es), 2969 steps, 7.0 h of pass time

category                       hours  share  steps  mean s
wait: other                     3.69  52.4%    225    59.0
model                           1.82  25.9%   1386     4.7
cargo test                      0.44   6.3%     94    16.9
cargo build                     0.36   5.1%     74    17.5
shell                           0.31   4.4%    565     2.0
phase mark                      0.10   1.4%     93     3.8
cargo publish                   0.08   1.1%     15    18.5
cargo clippy                    0.04   0.6%     12    11.8
git                             0.04   0.5%     53     2.6
github                          0.04   0.5%     39     3.5
remote wait                     0.04   0.5%      8    16.4
push gate                       0.03   0.4%     13     8.0
web                             0.02   0.3%     10     7.3
read files                      0.02   0.3%    252     0.3
search                          0.01   0.1%     33     0.9

phase                          hours  share  steps  mean s
build                           2.06  29.2%   1094     6.8
prove                           1.93  27.3%    713     9.7
gate                            1.59  22.5%    238    24.0
research                        0.46   6.5%    323     5.1
release                         0.31   4.5%     92    12.3
choose                          0.31   4.4%    254     4.4
ship                            0.22   3.1%     69    11.4
bugs                            0.06   0.8%     40     5.2
log                             0.06   0.8%     44     4.7
orient                          0.05   0.7%     80     2.3
done                            0.00   0.0%      2     3.8

slowest single steps
  pass  342     413s  bg    remote wait          bg_10 completed: while pgrep -f "cargo (nextest|test)" >/dev/null; do sleep 10; done; cat 
  pass   91     292s  tool  wait: other          {}
  pass  341     241s  tool  wait: other          {}
  pass  349     240s  tool  wait: other          {}
  pass  348     240s  tool  wait: other          {}
  pass  342     240s  tool  wait: other          {}
  pass  342     240s  tool  wait: other          {}
  pass  344     240s  tool  wait: other          {}
  pass  346     240s  tool  wait: other          {}
  pass  348     240s  tool  wait: other          {}
  pass  348     240s  tool  wait: other          {}
  pass  348     240s  tool  wait: other          {}
  pass  342     240s  tool  wait: other          {}
  pass  345     240s  tool  wait: other          {}
  pass  345     240s  tool  wait: other          {}
```

## Every pass recorded

```text
71 pass(es), 19719 steps, 53.9 h of pass time

category                       hours  share  steps  mean s
model                          20.48  38.0%   8980     8.2
wait: gate                      9.91  18.4%     74   482.3
wait: other                     8.23  15.3%    364    81.4
cargo test                      2.93   5.4%    596    17.7
wait: tests                     2.85   5.3%     34   302.0
shell                           2.23   4.1%   3353     2.4
cargo build                     1.93   3.6%    406    17.1
wait: build                     1.16   2.2%     21   199.4
phase mark                      1.03   1.9%    518     7.1
wait: clip                      0.79   1.5%     30    94.9
remote wait                     0.35   0.6%     45    28.0
read files                      0.28   0.5%   2336     0.4
cargo publish                   0.27   0.5%     73    13.5
github                          0.22   0.4%    212     3.8
cargo clippy                    0.22   0.4%     88     9.1

phase                          hours  share  steps  mean s
orient                         19.10  35.5%   4589    15.0
prove                           9.91  18.4%   4729     7.5
gate                            9.17  17.0%   1366    24.2
build                           8.94  16.6%   4825     6.7
research                        2.17   4.0%   1395     5.6
choose                          1.59   3.0%   1038     5.5
release                         1.35   2.5%    692     7.0
ship                            1.02   1.9%    537     6.8
log                             0.44   0.8%    339     4.7
bugs                            0.18   0.3%    111     5.9
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

