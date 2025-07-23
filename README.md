# `sums`: add things up

Sometimes you want to add a bunch of integers on the command line.
Stack overflow [has you covered](https://stackoverflow.com/questions/450799/shell-command-to-sum-integers-one-per-line)!
Except that neither of the top two solutions are simple or memorable.

Instead, just pipe your numbers to `sums`.

## Usage

```text
Usage: sums [INPUT_STREAM]

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

### Prebuilt (unix-ish)

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/coriolinus/sum/releases/latest/download/sum-installer.sh | sh
```

### Prebuilt (powershell)

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/coriolinus/sum/releases/latest/download/sum-installer.ps1 | iex"
```

### With Rust Toolchain

```sh
latest_release_tag="$(gh repo view coriolinus/sum --json latestRelease --jq '.latestRelease.tagName')"
cargo install --git https://github.com/coriolinus/sum --tag "$latest_release_tag"
```

## Contributing

Pull requests are welcome. Contributions imply permission to use said contribution for this project in the open-source way.

New release: `cargo release vx.y.z`.
