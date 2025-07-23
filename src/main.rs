//! Calculate the sum of a whitespace-separated set of numbers.
//!
//! Represents numbers as `f64`. If your sum exceeds `2**53`, you may lose accuracy.
//!
//! Conflicts with [Coreutils `sum`](https://www.gnu.org/software/coreutils/sum).
//! If that utility is important to you, consider renaming this binary.

mod input_stream;

use anyhow::Result;
use clap::Parser;

use crate::input_stream::InputStream;

/// Calculate the sum of a whitespace-separated set of numbers.
///
/// Numbers are read from the input stream until EOF. At that point, the sum is computed and printed to stdout.
///
/// All numbers are in base10. No spaces, underscores, commas or other symbols are permitted within a number.
#[derive(Debug, Parser)]
struct Args {
    /// File from which to read.
    ///
    /// `-` indicates to read from stdin.
    #[arg(default_value_t = InputStream::Stdin)]
    input_stream: InputStream,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut total = 0.0;
    for value in args.input_stream.into_iter()? {
        let value = value?;
        total += value;
    }

    if total.fract() == 0.0 {
        println!("{total:.0}")
    } else {
        println!("{total}")
    }

    Ok(())
}
