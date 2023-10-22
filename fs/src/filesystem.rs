use crate::directory::Directory;

/// A filesystem.
pub struct Filesystem {
  root: Directory,
}

impl Filesystem {
  /// Creates a new filesystem.
  #[must_use]
  pub fn new() -> Self {
    Self { root: Directory::root() }
  }
}

impl Default for Filesystem {
  fn default() -> Self {
    Self::new()
  }
}
