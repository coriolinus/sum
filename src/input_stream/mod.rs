mod iter;

use std::{
    fmt::{self, Write as _},
    fs::File,
    io::{BufReader, Read as _, stdin},
    path::PathBuf,
    str::FromStr,
};

use crate::input_stream::iter::{ByteIter, ParseFloatIterator};

#[derive(Debug, Clone)]
pub enum InputStream {
    Stdin,
    Path(PathBuf),
}

impl Default for InputStream {
    fn default() -> Self {
        Self::Stdin
    }
}

impl FromStr for InputStream {
    type Err = InputStreamError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(InputStreamError::Empty);
        }

        if s == "-" {
            return Ok(Self::Stdin);
        }

        let path = PathBuf::from(s);
        if !path.exists() {
            return Err(InputStreamError::DoesNotExist(path.display().to_string()));
        }
        if !path.is_file() {
            return Err(InputStreamError::NotFile(path.display().to_string()));
        }

        Ok(Self::Path(path))
    }
}

impl fmt::Display for InputStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InputStream::Stdin => f.write_char('-'),
            InputStream::Path(path) => path.display().fmt(f),
        }
    }
}

impl InputStream {
    pub fn into_iter(self) -> Result<ParseFloatIterator, std::io::Error> {
        let bytes = match self {
            InputStream::Stdin => Box::new(stdin().lock().bytes()) as ByteIter,
            InputStream::Path(path) => {
                let file = File::open(path)?;
                Box::new(BufReader::new(file).bytes())
            }
        };

        Ok(ParseFloatIterator::new(bytes))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum InputStreamError {
    #[error("input stream cannot be parsed from empty input")]
    Empty,
    #[error("path does not exist: {0}")]
    DoesNotExist(String),
    #[error("path is not file: {0}")]
    NotFile(String),
}
