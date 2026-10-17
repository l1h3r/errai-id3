//! Library Errors

use core::fmt::Display;
use core::fmt::Formatter;
use core::fmt::Result as FmtResult;
use core::num::TryFromIntError;
use std::error::Error as StdError;
use std::io::Error as IoError;

/// Alias for [`core::result::Result<T, E>`].
pub type Result<T, E = Error> = ::core::result::Result<T, E>;

// =============================================================================
// Error
// =============================================================================

/// Error returned from ID3 operations.
#[derive(Debug)]
pub struct Error {
  kind: ErrorKind,
  base: ErrorBase,
}

impl Error {
  pub(crate) const fn new(kind: ErrorKind) -> Self {
    Self {
      kind,
      base: ErrorBase::Ignore,
    }
  }

  pub(crate) fn new_std(kind: ErrorKind, source: impl StdError + Send + Sync + 'static) -> Self {
    Self {
      kind,
      base: ErrorBase::Source(Box::new(source)),
    }
  }

  pub(crate) const fn tag(field: TagField) -> Self {
    Self::new(ErrorKind::InvalidField(field))
  }

  pub(crate) const fn unsupported(feature: &'static str) -> Self {
    Self::new(ErrorKind::Unsupported(feature))
  }

  /// Get the category of the error.
  #[inline]
  pub const fn kind(&self) -> ErrorKind {
    self.kind
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    Display::fmt(&self.kind, f)?;

    if let ErrorBase::Source(ref inner) = self.base {
      write!(f, ": {inner}")?;
    }

    Ok(())
  }
}

impl StdError for Error {
  #[inline]
  fn source(&self) -> Option<&(dyn StdError + 'static)> {
    match self.base {
      ErrorBase::Ignore => None,
      ErrorBase::Source(ref inner) => Some(&**inner),
    }
  }
}

impl From<IoError> for Error {
  #[inline]
  fn from(other: IoError) -> Self {
    Self::new_std(ErrorKind::IO, other)
  }
}

impl From<TryFromIntError> for Error {
  #[inline]
  fn from(other: TryFromIntError) -> Self {
    Self::new_std(ErrorKind::Int, other)
  }
}

// =============================================================================
// Error Kind
// =============================================================================

/// The types of errors that may occur.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorKind {
  /// I/O error.
  IO,
  /// Integer conversion error.
  Int,
  /// Invalid ID3 tag field.
  InvalidField(TagField),
  /// Invalid ID3 tag version.
  InvalidVersion,
  /// Invalid bytes in frame identifier.
  InvalidFrameId,
  /// Invalid bitflag in frame header.
  InvalidBitFlag,
  /// Invalid data found in frame.
  InvalidFrameData,
  /// Valid ID3 feature that is not supported by this library.
  Unsupported(&'static str),
}

impl Display for ErrorKind {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::IO => f.write_str("i/o error"),
      Self::Int => f.write_str("integer conversion error"),
      Self::InvalidField(field) => write!(f, "invalid tag field: {field}"),
      Self::InvalidVersion => f.write_str("invalid tag version"),
      Self::InvalidFrameId => f.write_str("invalid frame identifier"),
      Self::InvalidBitFlag => f.write_str("invalid frame header flag"),
      Self::InvalidFrameData => f.write_str("invalid frame data"),
      Self::Unsupported(feature) => write!(f, "unsupported feature: {feature}"),
    }
  }
}

// =============================================================================
// Tag Field
// =============================================================================

/// Fields of an ID3v2 tag.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TagField {
  /// Header identifier.
  Identifier,
  /// Header version.
  Version,
  /// Extended header size.
  ExtSize,
  /// Extended header bitflag size.
  ExtFlagSize,
  /// Extended header flag data size.
  ExtFlagData,
}

impl Display for TagField {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::Identifier => f.write_str("identifier"),
      Self::Version => f.write_str("version"),
      Self::ExtSize => f.write_str("extended header size"),
      Self::ExtFlagSize => f.write_str("extended header flag size"),
      Self::ExtFlagData => f.write_str("extended header flag data"),
    }
  }
}

// =============================================================================
// Error Base
// =============================================================================

#[derive(Debug)]
enum ErrorBase {
  Ignore,
  Source(Box<dyn StdError + Send + Sync + 'static>),
}

impl Display for ErrorBase {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      Self::Ignore => Ok(()),
      Self::Source(inner) => Display::fmt(inner, f),
    }
  }
}
