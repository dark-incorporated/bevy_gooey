# Where bevy_gooey_loop's time goes

Written by `tools/loop/steps.py publish` after every pass; the rows are in [`bevy_gooey_loop.tsv`](bevy_gooey_loop.tsv). Updated 2026-11-25 23:47Z.

## What the last passes shipped

From [`bevy_gooey_loop-audit.tsv`](bevy_gooey_loop-audit.tsv): commits on main, unpushed commits, package files changed, release tags, doc and demo files, and what was missing.

| pass | at | commits | unpushed | package_files | released | docs_files | demo_files | missing |
|---|---|---|---|---|---|---|---|---|
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
| 352 | 2026-11-23T15:35Z | 0 | 0 | 0 | - | 0 | 0 | nothing shipped |
| 353 | 2026-11-25T23:47Z | 3 | 0 | 3 | v0.59.0 | 6 | 1 | - |

## How each configuration did

From [`bevy_gooey_loop-config.tsv`](bevy_gooey_loop-config.tsv): one row per pass with its host, wall time, model, thinking level and build settings.

```text
host       model                        think    jobs  threads  passes  mean min  median min
big        anthropic/claude-sonnet-5-5  high     8     8            11      41.7        36.4
big        anthropic/claude-sonnet-5-5  high     6     4            34      31.9        28.5
big        anthropic/claude-sonnet-5-5  high     3     4             8      37.0        33.9
big        ollama-cloud/deepseek-v4.1-flash:1017 high     3     4             2      49.1        49.1
```

## The last 12 passes

```text
12 pass(es), 3589 steps, 7.2 h of pass time

category                       hours  share  steps  mean s
wait: other                     3.64  50.2%    205    63.9
model                           1.88  25.9%   1680     4.0
cargo test                      0.57   7.9%    115    17.9
cargo build                     0.39   5.4%     84    16.7
shell                           0.34   4.7%    826     1.5
cargo publish                   0.10   1.4%     18    20.2
phase mark                      0.09   1.2%     85     3.8
git                             0.04   0.6%     95     1.6
remote wait                     0.04   0.5%     10    13.2
cargo clippy                    0.03   0.5%     13     9.4
github                          0.03   0.5%     43     2.8
push gate                       0.03   0.4%     15     6.2
web                             0.02   0.3%     10     7.3
read files                      0.02   0.2%    251     0.2
homestill                       0.01   0.2%     17     2.5

phase                          hours  share  steps  mean s
build                           1.61  22.2%    955     6.1
prove                           1.55  21.3%    627     8.9
research                        1.35  18.7%    921     5.3
gate                            1.25  17.2%    166    27.0
bugs                            0.64   8.8%    392     5.9
release                         0.31   4.3%     92    12.3
choose                          0.20   2.7%    181     4.0
ship                            0.18   2.5%     42    15.4
orient                          0.11   1.5%    144     2.6
log                             0.05   0.7%     42     4.6
done                            0.00   0.0%      2     3.8

slowest single steps
  pass  353     735s  bg    shell                bg_18 completed: cp /tmp/g117-fix.rs src/code_overlay.rs && export CARGO_TARGET_DIR=$HOME/
  pass  342     413s  bg    remote wait          bg_10 completed: while pgrep -f "cargo (nextest|test)" >/dev/null; do sleep 10; done; cat 
  pass  353     300s  bg    shell                bg_8 failed: export CARGO_TARGET_DIR=$HOME/.cargo-target-shared/global && unset CARGO_INCR
  pass  352     242s  bg    cargo test           bg_9 completed: RUSTC_WRAPPER="" CARGO_BUILD_RUSTC_WRAPPER="" CARGO_INCREMENTAL=0 CARGO_BU
  pass  353     242s  bg    shell                bg_11 completed: export CARGO_TARGET_DIR=$HOME/.cargo-target-shared/global && unset CARGO_
  pass  349     240s  tool  wait: other          {}
  pass  352     240s  tool  wait: other          {}
  pass  348     240s  tool  wait: other          {}
  pass  353     240s  tool  wait: other          {}
  pass  353     240s  tool  wait: other          {}
  pass  342     240s  tool  wait: other          {}
  pass  342     240s  tool  wait: other          {}
  pass  344     240s  tool  wait: other          {}
  pass  346     240s  tool  wait: other          {}
  pass  348     240s  tool  wait: other          {}
```

## Every pass recorded

```text
73 pass(es), 20810 steps, 55.5 h of pass time

category                       hours  share  steps  mean s
model                          20.96  37.8%   9498     7.9
wait: gate                      9.91  17.9%     74   482.3
wait: other                     9.05  16.3%    418    77.9
cargo test                      3.12   5.6%    633    17.7
wait: tests                     2.85   5.1%     34   302.0
shell                           2.28   4.1%   3701     2.2
cargo build                     1.98   3.6%    420    17.0
wait: build                     1.16   2.1%     21   199.4
phase mark                      1.03   1.9%    523     7.1
wait: clip                      0.79   1.4%     30    94.9
remote wait                     0.35   0.6%     47    26.8
cargo publish                   0.30   0.5%     77    13.9
read files                      0.28   0.5%   2352     0.4
github                          0.23   0.4%    223     3.7
cargo clippy                    0.22   0.4%     90     8.9

phase                          hours  share  steps  mean s
orient                         19.17  34.5%   4672    14.8
prove                           9.91  17.9%   4729     7.5
gate                            9.17  16.5%   1366    24.2
build                           8.94  16.1%   4825     6.7
research                        3.11   5.6%   2013     5.6
choose                          1.59   2.9%   1038     5.5
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

