/// Metadata about a [`Directory`] or [`File`].
///
/// [`Directory`]: crate::directory::Directory
/// [`File`]: crate::file::File
pub struct Metadata {
  name: String,
}

impl Metadata {
  /// Creates a new metadata.
  pub fn new<S: Into<String>>(name: S) -> Self {
    Self { name: name.into() }
  }
}
