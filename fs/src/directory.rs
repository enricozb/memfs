use crate::{file::File, metadata::Metadata};

/// A directory.
pub struct Directory {
  metadata: Metadata,

  entries: Vec<Entry>,
}

impl Directory {
  /// Creates a new directory.
  #[must_use]
  pub fn new<S: Into<String>>(name: S) -> Self {
    Self {
      metadata: Metadata::new(name),
      entries: Vec::new(),
    }
  }

  /// Creates a root directory.
  #[must_use]
  pub fn root() -> Self {
    Self::new("/")
  }
}

/// An entry in a directory.
pub enum Entry {
  File(File),
  Directory(Directory),
}
