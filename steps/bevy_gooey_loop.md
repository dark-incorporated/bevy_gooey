# Where bevy_gooey_loop's time goes

Written by `tools/loop/steps.py publish` after every pass; the rows are in [`bevy_gooey_loop.tsv`](bevy_gooey_loop.tsv). Updated 2026-11-26 00:04Z.

## What the last passes shipped

From [`bevy_gooey_loop-audit.tsv`](bevy_gooey_loop-audit.tsv): commits on main, unpushed commits, package files changed, release tags, doc and demo files, and what was missing.

| pass | at | commits | unpushed | package_files | released | docs_files | demo_files | missing |
|---|---|---|---|---|---|---|---|---|
| 343 | 2026-11-22T08:01Z | 0 | 0 | 0 | - | 0 | 0 | nothing shipped |
| 344 | 2026-11-22T08:44Z | 9 | 0 | 5 | - | 8 | 1 | release |
| 345 | 2026-11-22T09:40Z | 7 | 0 | 8 | v0.53.0,v0.54.0 | 6 | 1 | - |
| 346 | 2026-11-22T10:21Z | 7 | 0 | 6 | v0.55.0 | 8 | 1 | - |
| 347 | 2026-11-22T14:04Z | 8 | 0 | 4 | v0.56.0 | 6 | 1 | - |
| 348 | 2026-11-22T15:12Z | 7 | 0 | 6 | v0.57.0 | 6 | 1 | - |
| 349 | 2026-11-22T15:29Z | 0 | 0 | 0 | - | 0 | 0 | nothing shipped |
| 350 | 2026-11-23T00:06Z | 5 | 0 | 5 | v0.58.0 | 6 | 1 | - |
| 351 | 2026-11-23T00:30Z | 4 | 0 | 6 | - | 8 | 1 | release |
| 352 | 2026-11-23T15:35Z | 0 | 0 | 0 | - | 0 | 0 | nothing shipped |
| 353 | 2026-11-25T23:47Z | 3 | 0 | 3 | v0.59.0 | 6 | 1 | - |
| 354 | 2026-11-26T00:04Z | 0 | 0 | 0 | - | 0 | 0 | nothing shipped |

## How each configuration did

From [`bevy_gooey_loop-config.tsv`](bevy_gooey_loop-config.tsv): one row per pass with its host, wall time, model, thinking level and build settings.

```text
host       model                        think    jobs  threads  passes  mean min  median min
big        anthropic/claude-sonnet-5-5  high     8     8            11      41.7        36.4
big        anthropic/claude-sonnet-5-5  high     6     4            34      31.9        28.5
big        anthropic/claude-sonnet-5-5  high     3     4             8      37.0        33.9
big        ollama-cloud/deepseek-v4.1-flash:1017 high     3     4             3      38.4        35.6
```

## The last 12 passes

```text
12 pass(es), 3585 steps, 7.0 h of pass time

category                       hours  share  steps  mean s
wait: other                     3.36  47.8%    187    64.7
model                           2.05  29.2%   1652     4.5
cargo test                      0.53   7.6%    106    18.1
cargo build                     0.37   5.3%     82    16.4
shell                           0.32   4.6%    893     1.3
cargo publish                   0.08   1.2%     16    18.6
phase mark                      0.07   1.0%     79     3.3
git                             0.04   0.6%     91     1.7
remote wait                     0.04   0.5%      8    15.8
cargo clippy                    0.03   0.5%     13     9.4
github                          0.03   0.4%     44     2.6
push gate                       0.02   0.3%     12     7.0
web                             0.02   0.3%     10     7.3
read files                      0.02   0.2%    246     0.2
homestill                       0.01   0.2%     30     1.7

phase                          hours  share  steps  mean s
build                           1.54  22.0%    917     6.1
research                        1.47  20.9%    987     5.4
prove                           1.30  18.6%    543     8.6
gate                            1.11  15.8%    126    31.8
bugs                            0.64   9.1%    388     5.9
release                         0.27   3.9%     76    13.0
orient                          0.27   3.8%    273     3.5
choose                          0.20   2.8%    181     4.0
ship                            0.17   2.5%     36    17.3
log                             0.05   0.6%     34     4.8
done                            0.00   0.0%      2     3.8

slowest single steps
  pass  353     735s  bg    shell                bg_18 completed: cp /tmp/g117-fix.rs src/code_overlay.rs && export CARGO_TARGET_DIR=$HOME/
  pass  353     300s  bg    shell                bg_8 failed: export CARGO_TARGET_DIR=$HOME/.cargo-target-shared/global && unset CARGO_INCR
  pass  352     242s  bg    cargo test           bg_9 completed: RUSTC_WRAPPER="" CARGO_BUILD_RUSTC_WRAPPER="" CARGO_INCREMENTAL=0 CARGO_BU
  pass  353     242s  bg    shell                bg_11 completed: export CARGO_TARGET_DIR=$HOME/.cargo-target-shared/global && unset CARGO_
  pass  349     240s  tool  wait: other          {}
  pass  352     240s  tool  wait: other          {}
  pass  348     240s  tool  wait: other          {}
  pass  353     240s  tool  wait: other          {}
  pass  353     240s  tool  wait: other          {}
  pass  344     240s  tool  wait: other          {}
  pass  346     240s  tool  wait: other          {}
  pass  348     240s  tool  wait: other          {}
  pass  348     240s  tool  wait: other          {}
  pass  348     240s  tool  wait: other          {}
  pass  345     240s  tool  wait: other          {}
```

## Every pass recorded

```text
74 pass(es), 21011 steps, 55.8 h of pass time

category                       hours  share  steps  mean s
model                          21.24  38.1%   9569     8.0
wait: gate                      9.91  17.8%     74   482.3
wait: other                     9.05  16.2%    418    77.9
cargo test                      3.12   5.6%    633    17.7
wait: tests                     2.85   5.1%     34   302.0
shell                           2.29   4.1%   3803     2.2
cargo build                     1.98   3.6%    420    17.0
wait: build                     1.16   2.1%     21   199.4
phase mark                      1.03   1.8%    526     7.0
wait: clip                      0.79   1.4%     30    94.9
remote wait                     0.35   0.6%     47    26.8
cargo publish                   0.30   0.5%     77    13.9
read files                      0.28   0.5%   2352     0.4
github                          0.23   0.4%    230     3.6
cargo clippy                    0.22   0.4%     90     8.9

phase                          hours  share  steps  mean s
orient                         19.33  34.7%   4807    14.5
prove                           9.91  17.8%   4729     7.5
gate                            9.17  16.4%   1366    24.2
build                           8.94  16.0%   4825     6.7
research                        3.23   5.8%   2079     5.6
choose                          1.59   2.8%   1038     5.5
release                         1.35   2.4%    692     7.0
ship                            1.02   1.8%    537     6.8
bugs                            0.81   1.5%    494     5.9
log                             0.44   0.8%    339     4.7
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

