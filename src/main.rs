//! Calculate the sum of a whitespace-separated set of numbers.
//!
//! Represents numbers as `f64`. If your sum exceeds `2**53`, you may lose accuracy.

mod input_stream;

use anyhow::Result;
use clap::Parser;

use crate::input_stream::InputStream;

/// Below this value we know that every integer is distinct from every other.
/// Above this value, the error bars for things like addition are greater than 1.0.
const MAX_SAFE_INTEGER: f64 = (2_u64.pow(f64::MANTISSA_DIGITS) - 1) as _;

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

    if total.abs() > MAX_SAFE_INTEGER || total.fract() == 0.0 {
        println!("{total:.0}")
    } else {
        println!("{total}")
    }

    Ok(())
}
