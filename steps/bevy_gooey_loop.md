# Where bevy_gooey_loop's time goes

Written by `tools/loop/steps.py publish` after every pass; the rows are in [`bevy_gooey_loop.tsv`](bevy_gooey_loop.tsv). Updated 2026-11-26 01:05Z.

## What the last passes shipped

From [`bevy_gooey_loop-audit.tsv`](bevy_gooey_loop-audit.tsv): commits on main, unpushed commits, package files changed, release tags, doc and demo files, and what was missing.

| pass | at | commits | unpushed | package_files | released | docs_files | demo_files | missing |
|---|---|---|---|---|---|---|---|---|
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
| 355 | 2026-11-26T01:05Z | 5 | 0 | 3 | v0.60.0 | 10 | 2 | - |

## How each configuration did

From [`bevy_gooey_loop-config.tsv`](bevy_gooey_loop-config.tsv): one row per pass with its host, wall time, model, thinking level and build settings.

```text
host       model                        think    jobs  threads  passes  mean min  median min
big        anthropic/claude-sonnet-5-5  high     8     8            11      41.7        36.4
big        anthropic/claude-sonnet-5-5  high     6     4            34      31.9        28.5
big        anthropic/claude-sonnet-5-5  high     3     4             8      37.0        33.9
big        ollama-cloud/deepseek-v4.1-flash:1017 high     3     4             4      43.9        48.0
```

## The last 12 passes

```text
12 pass(es), 4201 steps, 7.9 h of pass time

category                       hours  share  steps  mean s
wait: other                     3.55  45.1%    198    64.5
model                           2.41  30.6%   1933     4.5
cargo test                      0.62   7.9%    109    20.5
cargo build                     0.44   5.6%    111    14.3
shell                           0.40   5.1%   1143     1.3
cargo publish                   0.11   1.5%     18    22.9
phase mark                      0.10   1.2%     89     3.9
git                             0.04   0.5%     92     1.7
cargo clippy                    0.04   0.5%     15     9.1
remote wait                     0.04   0.4%      8    15.8
github                          0.03   0.4%     45     2.5
push gate                       0.03   0.3%     15     6.2
homestill                       0.02   0.3%     35     2.2
web                             0.02   0.3%     10     7.4
read files                      0.02   0.3%    247     0.3

phase                          hours  share  steps  mean s
build                           1.87  23.8%   1187     5.7
research                        1.56  19.8%   1119     5.0
gate                            1.33  16.9%    178    26.9
prove                           1.31  16.6%    559     8.4
bugs                            0.63   8.0%    383     6.0
orient                          0.36   4.6%    376     3.5
release                         0.27   3.5%     76    13.0
choose                          0.25   3.2%    209     4.4
ship                            0.21   2.6%     44    16.9
log                             0.07   0.8%     44     5.5
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
  pass  355     240s  tool  wait: other          {}
  pass  344     240s  tool  wait: other          {}
  pass  346     240s  tool  wait: other          {}
  pass  348     240s  tool  wait: other          {}
  pass  348     240s  tool  wait: other          {}
  pass  348     240s  tool  wait: other          {}
```

## Every pass recorded

```text
75 pass(es), 21801 steps, 56.8 h of pass time

category                       hours  share  steps  mean s
model                          21.72  38.2%   9930     7.9
wait: gate                      9.91  17.5%     74   482.3
wait: other                     9.26  16.3%    434    76.8
cargo test                      3.21   5.6%    641    18.0
wait: tests                     2.85   5.0%     34   302.0
shell                           2.37   4.2%   4095     2.1
cargo build                     2.05   3.6%    449    16.4
wait: build                     1.16   2.0%     21   199.4
phase mark                      1.05   1.9%    540     7.0
wait: clip                      0.79   1.4%     30    94.9
remote wait                     0.35   0.6%     47    26.8
cargo publish                   0.33   0.6%     79    15.0
read files                      0.28   0.5%   2382     0.4
github                          0.23   0.4%    234     3.5
cargo clippy                    0.23   0.4%     92     8.9

phase                          hours  share  steps  mean s
orient                         19.43  34.2%   4913    14.2
prove                           9.92  17.5%   4745     7.5
gate                            9.38  16.5%   1418    23.8
build                           9.27  16.3%   5095     6.5
research                        3.42   6.0%   2307     5.3
choose                          1.71   3.0%   1136     5.4
release                         1.35   2.4%    692     7.0
ship                            1.05   1.9%    545     6.9
bugs                            0.81   1.4%    494     5.9
log                             0.47   0.8%    349     4.8
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

