# Where bevy_gooey_loop's time goes

Written by `tools/loop/steps.py publish` after every pass; the rows are in [`bevy_gooey_loop.tsv`](bevy_gooey_loop.tsv). Updated 2026-11-27 23:51Z.

## What the last passes shipped

From [`bevy_gooey_loop-audit.tsv`](bevy_gooey_loop-audit.tsv): commits on main, unpushed commits, package files changed, release tags, doc and demo files, and what was missing.

| pass | at | commits | unpushed | package_files | released | docs_files | demo_files | missing |
|---|---|---|---|---|---|---|---|---|
| 381 | 2026-11-27T18:08Z | 0 | 0 | 0 | - | 0 | 0 | nothing shipped |
| 382 | 2026-11-27T18:10Z | 0 | 0 | 0 | - | 0 | 0 | nothing shipped |
| 383 | 2026-11-27T18:13Z | 0 | 0 | 0 | - | 0 | 0 | nothing shipped |
| 384 | 2026-11-27T18:26Z | 0 | 0 | 0 | - | 0 | 0 | nothing shipped |
| 385 | 2026-11-27T18:28Z | 0 | 0 | 0 | - | 0 | 0 | nothing shipped |
| 386 | 2026-11-27T18:30Z | 0 | 0 | 0 | - | 0 | 0 | nothing shipped |
| 387 | 2026-11-27T18:32Z | 0 | 0 | 0 | - | 0 | 0 | nothing shipped |
| 388 | 2026-11-27T18:44Z | 0 | 0 | 0 | - | 0 | 0 | nothing shipped |
| 389 | 2026-11-27T18:57Z | 0 | 0 | 0 | - | 0 | 0 | nothing shipped |
| 392 | 2026-11-27T20:15Z | 8 | 0 | 3 | v0.64.1 | 6 | 1 | - |
| 393 | 2026-11-27T22:17Z | 3 | 0 | 3 | v0.64.2 | 6 | 1 | - |
| 395 | 2026-11-27T23:51Z | 5 | 0 | 4 | v0.65.0 | 6 | 1 | - |

## How each configuration did

From [`bevy_gooey_loop-config.tsv`](bevy_gooey_loop-config.tsv): one row per pass with its host, wall time, model, thinking level and build settings.

```text
host       model                        think    jobs  threads  passes  mean min  median min
big        anthropic/claude-sonnet-5-5  high     8     8            11      41.7        36.4
big        anthropic/claude-sonnet-5-5  high     6     4            34      31.9        28.5
big        anthropic/claude-sonnet-5-5  high     3     4             8      37.0        33.9
big        ollama-cloud/deepseek-v4.1-flash:1017 high     3     4            41      18.3         0.5
```

## The last 12 passes

```text
12 pass(es), 1739 steps, 3.7 h of pass time

category                       hours  share  steps  mean s
model                           1.45  39.3%    801     6.5
wait: other                     1.34  36.3%     65    74.3
cargo test                      0.32   8.6%     33    34.8
shell                           0.17   4.7%    592     1.1
cargo build                     0.12   3.1%     29    14.4
homestill                       0.08   2.3%     31     9.8
remote wait                     0.06   1.5%      5    41.1
cargo publish                   0.04   1.1%      7    20.5
phase mark                      0.03   0.9%     24     4.7
cargo clippy                    0.02   0.6%      3    26.5
push gate                       0.02   0.6%     10     7.6
git                             0.02   0.5%     54     1.3
github                          0.01   0.3%     29     1.3
web                             0.00   0.1%      2     7.8
read files                      0.00   0.1%     38     0.2

phase                          hours  share  steps  mean s
research                        0.64  17.2%    336     6.8
orient                          0.59  16.0%    133    16.0
gate                            0.55  15.0%     94    21.2
prove                           0.48  13.0%    378     4.6
build                           0.46  12.5%    294     5.6
bugs                            0.42  11.5%    273     5.6
log                             0.32   8.6%     68    16.9
choose                          0.13   3.5%    111     4.3
ship                            0.10   2.7%     48     7.4

slowest single steps
  pass  392     332s  bg    shell                bg_13 completed: for i in $(seq 1 55); do if grep -q 'GATE DONE' /tmp/g116/gate.log; then 
  pass  395     242s  tool  wait: other          {}
  pass  395     241s  tool  wait: other          {}
  pass  395     241s  tool  wait: other          {}
  pass  393     240s  tool  wait: other          {"timeout": 900}
  pass  393     240s  tool  wait: other          {"timeout": 600}
  pass  395     240s  tool  wait: other          {}
  pass  392     240s  tool  wait: other          {}
  pass  393     240s  tool  wait: other          {}
  pass  393     240s  tool  wait: other          {"timeout": 600}
  pass  395     240s  tool  wait: other          {}
  pass  393     225s  tool  wait: other          {}
  pass  393     208s  tool  wait: other          {}
  pass  392     202s  bg    shell                bg_6 completed: echo "CARGO_INCREMENTAL=$CARGO_INCREMENTAL"; export CARGO_TARGET_DIR=/home
  pass  392     192s  tool  wait: other          {}
```

## Every pass recorded

```text
112 pass(es), 26169 steps, 66.4 h of pass time

category                       hours  share  steps  mean s
model                          25.30  38.1%  11985     7.6
wait: other                    12.29  18.5%    626    70.7
wait: gate                      9.91  14.9%     74   482.3
cargo test                      4.62   7.0%    794    20.9
wait: tests                     2.85   4.3%     34   302.0
shell                           2.82   4.2%   5474     1.9
cargo build                     2.60   3.9%    521    18.0
wait: build                     1.16   1.8%     21   199.4
phase mark                      1.11   1.7%    594     6.7
wait: clip                      0.79   1.2%     30    94.9
remote wait                     0.46   0.7%     59    27.9
cargo publish                   0.44   0.7%     97    16.3
read files                      0.29   0.4%   2474     0.4
cargo clippy                    0.28   0.4%     98    10.4
homestill                       0.26   0.4%    173     5.5

phase                          hours  share  steps  mean s
orient                         21.01  31.6%   5420    14.0
prove                          10.76  16.2%   5299     7.3
build                          10.63  16.0%   5791     6.6
gate                           10.35  15.6%   1604    23.2
research                        6.18   9.3%   3668     6.1
choose                          2.06   3.1%   1427     5.2
log                             1.57   2.4%    732     7.7
release                         1.39   2.1%    722     7.0
bugs                            1.30   2.0%    784     6.0
ship                            1.15   1.7%    597     6.9
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

