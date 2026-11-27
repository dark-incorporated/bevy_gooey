# Where bevy_gooey_loop's time goes

Written by `tools/loop/steps.py publish` after every pass; the rows are in [`bevy_gooey_loop.tsv`](bevy_gooey_loop.tsv). Updated 2026-11-27 22:17Z.

## What the last passes shipped

From [`bevy_gooey_loop-audit.tsv`](bevy_gooey_loop-audit.tsv): commits on main, unpushed commits, package files changed, release tags, doc and demo files, and what was missing.

| pass | at | commits | unpushed | package_files | released | docs_files | demo_files | missing |
|---|---|---|---|---|---|---|---|---|
| 380 | 2026-11-27T18:06Z | 0 | 0 | 0 | - | 0 | 0 | nothing shipped |
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

## How each configuration did

From [`bevy_gooey_loop-config.tsv`](bevy_gooey_loop-config.tsv): one row per pass with its host, wall time, model, thinking level and build settings.

```text
host       model                        think    jobs  threads  passes  mean min  median min
big        anthropic/claude-sonnet-5-5  high     8     8            11      41.7        36.4
big        anthropic/claude-sonnet-5-5  high     6     4            34      31.9        28.5
big        anthropic/claude-sonnet-5-5  high     3     4             8      37.0        33.9
big        ollama-cloud/deepseek-v4.1-flash:1017 high     3     4            40      17.2         0.5
```

## The last 12 passes

```text
12 pass(es), 1209 steps, 2.6 h of pass time

category                       hours  share  steps  mean s
model                           1.12  42.8%    573     7.0
wait: other                     0.89  34.2%     48    66.9
cargo test                      0.21   7.8%     21    35.2
shell                           0.15   5.6%    414     1.3
cargo build                     0.11   4.2%     22    17.9
remote wait                     0.06   2.2%      5    41.1
push gate                       0.02   0.7%      5    13.4
cargo publish                   0.02   0.6%      4    14.1
homestill                       0.02   0.6%      4    13.7
git                             0.01   0.5%     37     1.3
phase mark                      0.01   0.4%     15     2.6
github                          0.00   0.2%     19     0.8
read files                      0.00   0.1%     27     0.3
web                             0.00   0.1%      1     7.3
sleep                           0.00   0.0%      1     2.0

phase                          hours  share  steps  mean s
orient                          0.57  21.8%    115    17.9
build                           0.46  17.6%    294     5.6
bugs                            0.38  14.6%    220     6.2
gate                            0.38  14.4%     48    28.2
prove                           0.37  14.1%    284     4.7
log                             0.24   9.0%     56    15.2
research                        0.12   4.4%    109     3.8
choose                          0.07   2.6%     69     3.5
ship                            0.03   1.3%     10    12.2

slowest single steps
  pass  392     332s  bg    shell                bg_13 completed: for i in $(seq 1 55); do if grep -q 'GATE DONE' /tmp/g116/gate.log; then 
  pass  393     240s  tool  wait: other          {"timeout": 900}
  pass  393     240s  tool  wait: other          {"timeout": 600}
  pass  392     240s  tool  wait: other          {}
  pass  393     240s  tool  wait: other          {}
  pass  393     240s  tool  wait: other          {"timeout": 600}
  pass  393     225s  tool  wait: other          {}
  pass  393     208s  tool  wait: other          {}
  pass  392     202s  bg    shell                bg_6 completed: echo "CARGO_INCREMENTAL=$CARGO_INCREMENTAL"; export CARGO_TARGET_DIR=/home
  pass  392     192s  tool  wait: other          {}
  pass  392     189s  tool  wait: other          {}
  pass  392     186s  tool  wait: other          {}
  pass  392     161s  bg    cargo test           bg_7 completed: export CARGO_TARGET_DIR=/home/ladvien/target/gooey-loop CARGO_INCREMENTAL=
  pass  393     152s  tool  wait: other          {}
  pass  393     118s  tool  wait: other          {}
```

## Every pass recorded

```text
111 pass(es), 25638 steps, 65.3 h of pass time

category                       hours  share  steps  mean s
model                          24.97  38.2%  11756     7.6
wait: other                    11.84  18.1%    609    70.0
wait: gate                      9.91  15.2%     74   482.3
cargo test                      4.51   6.9%    782    20.7
wait: tests                     2.85   4.4%     34   302.0
shell                           2.79   4.3%   5296     1.9
cargo build                     2.59   4.0%    514    18.2
wait: build                     1.16   1.8%     21   199.4
phase mark                      1.09   1.7%    585     6.7
wait: clip                      0.79   1.2%     30    94.9
remote wait                     0.46   0.7%     59    27.9
cargo publish                   0.41   0.6%     94    15.8
read files                      0.29   0.4%   2463     0.4
cargo clippy                    0.26   0.4%     95     9.9
github                          0.25   0.4%    292     3.1

phase                          hours  share  steps  mean s
orient                         20.99  32.1%   5401    14.0
prove                          10.65  16.3%   5205     7.4
build                          10.63  16.3%   5791     6.6
gate                           10.18  15.6%   1558    23.5
research                        5.66   8.7%   3441     5.9
choose                          2.00   3.1%   1385     5.2
log                             1.48   2.3%    720     7.4
release                         1.39   2.1%    722     7.0
bugs                            1.26   1.9%    731     6.2
ship                            1.09   1.7%    559     7.0
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

