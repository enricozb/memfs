use std::ffi::OsString;

use chrono::{DateTime, Utc};

/// Metadata about a [`Directory`] or [`File`].
///
/// [`Directory`]: crate::directory::Directory
/// [`File`]: crate::file::File
pub struct Metadata {
  pub name: OsString,

  pub created_at: DateTime<Utc>,
}

impl Metadata {
  /// Creates a new metadata.
  pub fn new<S: Into<OsString>>(name: S) -> Self {
    Self {
      name: name.into(),
      created_at: Utc::now(),
    }
  }
}
