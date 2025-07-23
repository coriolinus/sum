use std::{num::ParseFloatError, str::Utf8Error};

use arrayvec::ArrayVec;

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

            try_!(
                buffer
                    .try_push(byte)
                    .map_err(|_| ParseFloatIteratorError::TooBig)
            );
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
    #[error("too many digits in input; failed to parse")]
    TooBig,
    #[error("input was not utf-8")]
    NotUtf8(#[from] Utf8Error),
    #[error("failed to parse \"{input}\"")]
    ParseFloat {
        #[source]
        err: ParseFloatError,
        input: String,
    },
}
