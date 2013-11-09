This is a library for performing speculative execution, based off of [this paper](http://research.microsoft.com/pubs/118795/pldi026-vaswani.pdf). It is written for Rust 0.8 and compiles with rustpkg.

# Usage

There are two functions provided in the `speculate` library: `spec` and `specfold`.

## Simple, two-task speculation

The `spec` function looks like this:

```rust
fn spec<A: Eq + Send + Clone, B>(producer: ~fn() -> A,
                                 predictor: ~fn() -> A,
                                 consumer:  ~fn(A) -> B) -> B
```

Both the `producer` and `consumer` functions are assumed to take a long time to run, while the `predictor` should be quick. The `producer` is run in its own task, then the `predictor` comes up with a guess, passes it to the `consumer`, which is run. Once the `producer` finishes, its result is compared to that of the `predictor`. If they match, then the result produced by the `consumer` is returned. If they do not match, then the `consumer` is re-run with the actual result from the `producer`.

Here is an adapted illustration from a [Haskell implementation](http://hackage.haskell.org/package/speculation) of the same paper. `p` is the `producer`, `g` is the `predictor` (guess), `c` is `consumer`, and `a` is the actual value returned by the `producer`.

The best-case timeline looks like:

```
foreground: [----- p -----]
foreground:               [-]    (check g == a)
spark:         [----- c g -----]
overall:    [--- spec p g c ---]
```

The worst-case timeline looks like:

```
foreground: [----- p -----]
foreground:               [-]              (check g == a)
foreground:                 [---- c a ----]
spark:         [----- c g -----]
overall:    [-------- spec p g c ---------]
```

## Iterative speculation

The `specfold` function launches a configurable number of tasks to work in parallel. It looks like this:

```rust
fn specfold<A: Eq + Clone + Send>(iters: uint,
                                  loop_body: &fn() -> ~fn(int, A) -> A,
                                  predictor: &fn() -> ~fn(int) -> A)
```

In this case, `iters` tasks are spawned which each execute an iteration of `loop_body`. Each task executes `loop_body()(idx, predictor()(idx))` in parallel. After all tasks have been launched, the main task sequentially checks each of the predictions and re-runs the loop body if a prediction was incorrect. A future version may attempt to do this in parallel.

The reason `predictor` and `loop_body` are functions which return other functions is because I couldn't get Rust's compiler to leave me alone otherwise.

# CSS parser

A modified version of [rust-cssparser](https://github.com/mozilla-servo/rust-cssparser/) is included and is used as a more real-world test of the library. The original version mixes tokenization with parsing, which is fine in the single-threaded case, but doesn't work as well here. The version included does only tokenization, which is useful when trying to parallelize. The `spec_css` library implements a speculative lexer using `specfold`.

## Benchmarking the lexer

If you put CSS files in a folder called `sample-data` at the project root and run the executable produced by the `testing` library, it will, for each file, run the lexer sequentially and in parallel and write to stdout a CSV file. The CSV file has columns `name, seq, par, size`, where `seq` and `par` are the time taken (in microseconds) to tokenize the file sequentially and in parallel, respectively, and `size` is the size of the file in bytes.

The number of tasks is controlled by providing a command-line argument like so (for 6 tasks):

```
./build/x86_64-unknown-linux-gnu/testing/testing 6
```

The default number of tasks is 4.

## Sample benchmark results

Here are the results from tokenizing the CSS files from the Alexa top 11 (I felt like including Amazon) on my Core 2 Quad Q6600 @ 2.40GHz with 4 tasks. This is the speedup of the parallel version compared to the sequential, sorted by file size (in KiB):

name                    | speedup |   size
------------------------|---------|:-------
linkedin00.css          |   1.837 |  418.3
youtube01.css           |   0.986 | 293.04
facebook04.css          |   1.865 | 244.61
linkedin05.css          |   1.229 | 220.54
linkedin02.css          |   1.848 | 199.26
twitter03.css           |    1.84 | 161.32
yahoo04.css             |   1.848 |  154.4
youtube02.css           |   1.662 | 133.97
linkedin01.css          |   1.884 | 122.45
bootstrap.css           |   1.585 | 117.08
wikipedia06.css         |    1.31 | 116.46
outlook12.css           |   0.774 | 116.42
amazon14.css            |   1.866 | 108.49
outlook14.css           |   1.642 |  96.09
bootstrap.min.css       |   1.888 |  95.06
facebook07.css          |   1.419 |  77.38
qq00.css                |   1.757 |  68.29
wikipedia02.css         |   0.867 |  66.18
facebook01.css          |   1.947 |  52.32
facebook03.css          |   1.588 |  48.96
google08.css            |    1.32 |  45.02
google00.css            |   1.241 |     42
facebook09.css          |   1.626 |  40.71
yahoo05.css             |   1.376 |  40.11
wikipedia01.css         |   0.857 |  39.18
outlook01.css           |   1.387 |  35.95
facebook00.css          |   1.843 |  33.78
outlook10.css           |   1.413 |  30.11
linkedin04.css          |   1.282 |  25.45
facebook08.css          |   1.288 |  25.32
wikipedia03.css         |   0.968 |  25.08
amazon13.css            |   1.199 |  22.34
amazon15.css            |   1.508 |  21.88
linkedin03.css          |   1.925 |  21.73
wikipedia07.css         |   1.589 |  20.56
outlook06.css           |   0.991 |  18.62
bootstrap-theme.css     |   1.748 |  16.42
google03.css            |   1.356 |  16.33
outlook05.css           |   1.876 |  16.31
bootstrap-theme.min.css |    1.89 |  14.64
amazon00.css            |   1.154 |  13.98
outlook15.css           |   1.254 |  13.94
amazon11.css            |   1.573 |   8.53
qq04.css                |   0.896 |    8.4
google07.css            |   0.803 |   7.64
wikipedia00.css         |   1.611 |   7.21
twitter00.css           |   1.546 |   5.76
outlook07.css           |    1.14 |   5.66
outlook18.css           |   1.489 |      5
outlook13.css           |   1.141 |   4.92
youtube00.css           |   1.184 |   4.76
amazon16.css            |   1.492 |   4.41
outlook17.css           |   1.552 |    3.7
baidu00.css             |   1.534 |   3.25
amazon09.css            |   1.106 |   3.19
google02.css            |    1.44 |   3.05
google01.css            |   1.041 |   2.76
qq05.css                |   1.453 |   2.76
google06.css            |   1.069 |   2.53
outlook02.css           |   1.439 |   2.41
amazon03.css            |     1.1 |   2.28
amazon01.css            |   1.227 |   2.09
yahoo03.css             |   1.003 |   1.96
yahoo06.css             |   1.001 |   1.81
google05.css            |   0.986 |   1.73
outlook08.css           |   0.897 |   1.34
amazon12.css            |   0.987 |   1.32
outlook00.css           |   0.904 |   1.26
baidu01.css             |   1.092 |   0.97
amazon06.css            |   0.571 |   0.95
yahoo00.css             |   0.715 |   0.95
yahoo02.css             |   0.827 |   0.74
outlook16.css           |   0.975 |   0.73
facebook02.css          |   0.758 |   0.68
facebook05.css          |   0.608 |   0.57
amazon10.css            |   0.647 |    0.5
amazon04.css            |   0.767 |   0.49
outlook11.css           |   0.668 |   0.45
facebook06.css          |   0.564 |   0.42
google04.css            |   0.534 |   0.41
outlook09.css           |   0.526 |   0.36
amazon02.css            |   0.504 |   0.32
qq01.css                |    0.61 |   0.31
amazon08.css            |   0.461 |   0.27
amazon05.css            |   0.373 |   0.22
qq02.css                |   0.365 |   0.18
wikipedia04.css         |     0.2 |   0.18
yahoo01.css             |    0.39 |   0.18
twitter01.css           |   0.163 |   0.11
amazon07.css            |   0.223 |   0.11
twitter02.css           |   0.124 |   0.07
wikipedia05.css         |    0.13 |   0.07
outlook03.css           |   0.131 |   0.06
qq03.css                |   0.108 |   0.03
outlook04.css           |   0.074 |   0.02
