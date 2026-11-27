# Where bevy_gooey_loop's time goes

Written by `tools/loop/steps.py publish` after every pass; the rows are in [`bevy_gooey_loop.tsv`](bevy_gooey_loop.tsv). Updated 2026-11-27 20:15Z.

## What the last passes shipped

From [`bevy_gooey_loop-audit.tsv`](bevy_gooey_loop-audit.tsv): commits on main, unpushed commits, package files changed, release tags, doc and demo files, and what was missing.

| pass | at | commits | unpushed | package_files | released | docs_files | demo_files | missing |
|---|---|---|---|---|---|---|---|---|
| 379 | 2026-11-27T18:04Z | 0 | 0 | 0 | - | 0 | 0 | nothing shipped |
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

## How each configuration did

From [`bevy_gooey_loop-config.tsv`](bevy_gooey_loop-config.tsv): one row per pass with its host, wall time, model, thinking level and build settings.

```text
host       model                        think    jobs  threads  passes  mean min  median min
big        anthropic/claude-sonnet-5-5  high     8     8            11      41.7        36.4
big        anthropic/claude-sonnet-5-5  high     6     4            34      31.9        28.5
big        anthropic/claude-sonnet-5-5  high     3     4             8      37.0        33.9
big        ollama-cloud/deepseek-v4.1-flash:1017 high     3     4            39      15.4         0.5
```

## The last 12 passes

```text
12 pass(es), 502 steps, 1.1 h of pass time

category                       hours  share  steps  mean s
model                           0.69  61.3%    261     9.5
wait: other                     0.24  21.3%     12    72.2
cargo test                      0.11  10.0%     12    34.0
shell                           0.03   2.6%    134     0.8
cargo build                     0.02   1.9%      7    11.0
git                             0.01   1.1%     34     1.4
cargo publish                   0.01   0.7%      3     9.5
phase mark                      0.01   0.5%      5     4.2
github                          0.00   0.2%     11     0.9
push gate                       0.00   0.2%      4     1.8
read files                      0.00   0.0%      7     0.1
edit files                      0.00   0.0%      8     0.0

phase                          hours  share  steps  mean s
orient                          0.54  48.0%     62    31.5
log                             0.23  20.7%     54    15.6
prove                           0.16  13.8%    150     3.7
build                           0.16  13.8%    180     3.1
choose                          0.04   3.7%     52     2.9

slowest single steps
  pass  392     332s  bg    shell                bg_13 completed: for i in $(seq 1 55); do if grep -q 'GATE DONE' /tmp/g116/gate.log; then 
  pass  392     240s  tool  wait: other          {}
  pass  392     202s  bg    shell                bg_6 completed: echo "CARGO_INCREMENTAL=$CARGO_INCREMENTAL"; export CARGO_TARGET_DIR=/home
  pass  392     192s  tool  wait: other          {}
  pass  392     189s  tool  wait: other          {}
  pass  392     186s  tool  wait: other          {}
  pass  392     161s  bg    cargo test           bg_7 completed: export CARGO_TARGET_DIR=/home/ladvien/target/gooey-loop CARGO_INCREMENTAL=
  pass  392     107s  bg    cargo test           bg_8 completed: export CARGO_TARGET_DIR=/home/ladvien/target/gooey-loop CARGO_INCREMENTAL=
  pass  384      64s  model model                
  pass  384      63s  model model                
  pass  389      63s  model model                
  pass  384      61s  model model                
  pass  389      61s  model model                
  pass  384      61s  model model                
  pass  384      61s  model model                
```

## Every pass recorded

```text
110 pass(es), 24930 steps, 63.9 h of pass time

category                       hours  share  steps  mean s
model                          24.54  38.4%  11443     7.7
wait: other                    11.18  17.5%    573    70.3
wait: gate                      9.91  15.5%     74   482.3
cargo test                      4.41   6.9%    773    20.6
wait: tests                     2.85   4.5%     34   302.0
shell                           2.67   4.2%   5016     1.9
cargo build                     2.50   3.9%    499    18.1
wait: build                     1.16   1.8%     21   199.4
phase mark                      1.08   1.7%    575     6.8
wait: clip                      0.79   1.2%     30    94.9
cargo publish                   0.41   0.6%     93    15.7
remote wait                     0.40   0.6%     54    26.6
read files                      0.29   0.5%   2443     0.4
cargo clippy                    0.26   0.4%     95     9.9
github                          0.25   0.4%    284     3.2

phase                          hours  share  steps  mean s
orient                         20.96  32.8%   5347    14.1
prove                          10.43  16.3%   5071     7.4
build                          10.33  16.2%   5677     6.5
gate                            9.80  15.3%   1510    23.4
research                        5.55   8.7%   3332     6.0
choose                          1.97   3.1%   1368     5.2
log                             1.48   2.3%    718     7.4
release                         1.39   2.2%    722     7.0
ship                            1.05   1.6%    549     6.9
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

