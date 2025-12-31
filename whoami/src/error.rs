use alloc::borrow::Cow;
#[cfg(feature = "std")]
use std::io::Error as IoError;

#[cfg(not(feature = "std"))]
#[derive(Clone, PartialEq, Eq, Debug)]
struct IoError(Cow<'static, str>);

/// An I/O error; can be converted to [`std::io::Error`].
#[non_exhaustive]
#[derive(Debug)]
pub struct Error(IoError);

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(&self, f)
    }
}

impl core::error::Error for Error {}

#[allow(dead_code)]
impl Error {
    fn new(message: &'static str) -> Self {
        #[cfg(not(feature = "std"))]
        {
            Self::from_io(IoError(message.into()))
        }

        #[cfg(feature = "std")]
        {
            Self::from_io(IoError::new(std::io::ErrorKind::NotFound, message))
        }
    }

    pub(crate) fn with_invalid_data(
        message: impl Into<Cow<'static, str>>,
    ) -> Self {
        let message = message.into();

        #[cfg(not(feature = "std"))]
        {
            Self::from_io(IoError(message))
        }

        #[cfg(feature = "std")]
        {
            Self::from_io(IoError::new(
                std::io::ErrorKind::InvalidData,
                message,
            ))
        }
    }

    pub(crate) fn from_io(err: IoError) -> Self {
        Self(err)
    }

    pub(crate) fn missing_record() -> Self {
        Self::new("Missing record")
    }

    pub(crate) fn null_record() -> Self {
        Self::new("Null record")
    }

    pub(crate) fn empty_record() -> Self {
        Self::new("Empty record")
    }
}

#[cfg(feature = "std")]
impl From<Error> for IoError {
    fn from(err: Error) -> Self {
        err.0
    }
}
