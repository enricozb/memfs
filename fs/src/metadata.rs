use std::ffi::OsString;

/// Metadata about a [`Directory`] or [`File`].
///
/// [`Directory`]: crate::directory::Directory
/// [`File`]: crate::file::File
pub struct Metadata {
  pub name: OsString,
}

impl Metadata {
  /// Creates a new metadata.
  pub fn new<S: Into<OsString>>(name: S) -> Self {
    Self { name: name.into() }
  }
}
