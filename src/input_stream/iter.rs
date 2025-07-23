use std::{num::ParseFloatError, str::Utf8Error};

use arrayvec::ArrayVec;
use bstr::BString;

/// Like the `?` operator, but wraps its output in `Some`
macro_rules! try_ {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => return Some(Err(e.into())),
        }
    };
}

pub(crate) type ByteIter = Box<dyn Iterator<Item = Result<u8, std::io::Error>>>;

/// How many digits we attempt to store to parse.
///
/// Numbers represented with more digits than this will cause an error.
///
/// This is somewhat arbitrarily chosen as twice as many digits as max integer precision;
/// at that point, we're already losing a fair amount of precision, but we can still approximate.
///
/// The point is that if we get a sequence of bytes that accidentally happens to be megabytes
/// of ascii digits, we don't attempt to store/parse them all.
const PARSE_CAPACITY: usize = (f64::DIGITS * 2) as _;

pub struct ParseFloatIterator {
    bytes: ByteIter,
}

impl ParseFloatIterator {
    pub(crate) fn new(bytes: ByteIter) -> Self {
        Self { bytes }
    }
}

impl Iterator for ParseFloatIterator {
    type Item = Result<f64, ParseFloatIteratorError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = ArrayVec::<u8, PARSE_CAPACITY>::new();

        for byte in self.bytes.by_ref() {
            let byte = try_!(byte);
            if byte.is_ascii_whitespace() {
                if buffer.is_empty() {
                    // scan through any amount of whitespace while the buffer is empty
                    continue;
                } else {
                    // once we have a non-empty buffer, the next whitespace byte is terminal
                    break;
                }
            }

            try_!(buffer.try_push(byte).map_err(|_| {
                ParseFloatIteratorError::TooBig {
                    input: BString::new(
                        buffer
                            .take()
                            .into_inner()
                            .expect("if this were not full we would not be here")
                            .into(),
                    ),
                }
            }));
        }

        if buffer.is_empty() {
            // EOF
            return None;
        }

        let input = try_!(str::from_utf8(&buffer));
        let value = try_!(
            input
                .parse()
                .map_err(|err| ParseFloatIteratorError::ParseFloat {
                    err,
                    input: input.to_owned(),
                })
        );
        Some(Ok(value))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ParseFloatIteratorError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("too many digits in input; failed to parse \"{input}…\"")]
    TooBig { input: BString },
    #[error("input was not utf-8")]
    NotUtf8(#[from] Utf8Error),
    #[error("failed to parse \"{input}\"")]
    ParseFloat {
        #[source]
        err: ParseFloatError,
        input: String,
    },
}
