use std::ffi::OsString;

use crate::metadata::Metadata;

/// A file.
pub struct File {
  pub metadata: Metadata,

  pub content: Vec<u8>,
}

impl File {
  /// Creates a new file.
  #[must_use]
  pub fn new<S: Into<OsString>>(name: S) -> Self {
    Self {
      metadata: Metadata::new(name),
      content: Vec::new(),
    }
  }
}
