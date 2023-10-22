use std::{collections::BTreeMap, ffi::OsString};

use crate::{metadata::Metadata, Entry};

/// A directory.
pub struct Directory {
  pub metadata: Metadata,

  pub entries: BTreeMap<OsString, Entry>,
}

impl Directory {
  /// Creates a new directory.
  #[must_use]
  pub fn new<S: Into<OsString>>(name: S) -> Self {
    Self {
      metadata: Metadata::new(name),
      entries: BTreeMap::new(),
    }
  }

  /// Creates a root directory.
  #[must_use]
  pub fn root() -> Self {
    Self::new("/")
  }
}
