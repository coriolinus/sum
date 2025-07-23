# `sums`: add things up

Sometimes you want to add a bunch of integers on the command line.
Stack overflow [has you covered](https://stackoverflow.com/questions/450799/shell-command-to-sum-integers-one-per-line)!
Except that neither of the top two solutions are simple or memorable.

Instead, just pipe your numbers to `sums`.

## Usage

```text
Usage: sum [INPUT_STREAM]

Arguments:
  [INPUT_STREAM]
          File from which to read.

          `-` indicates to read from stdin.

          [default: -]

Options:
  -h, --help
          Print help (see a summary with '-h')
```

## Features

- not limited to 1 number per line; any ascii whitespace is an acceptable separator
- not limited to integers; floats work fine
- not limited to stdin; can work on files
- streams through the input for minimal memory consumption
- easy to remember

## Installation

```sh
cargo install --git https://github.com/coriolinus/sum
```
