pub mod error;
pub mod util;

use std::path::{Component, Path, PathBuf};

use fs::{entry::Borrowed as BorrowedEntry, Filesystem};

pub use self::error::{Error, Result};

/// An interactive session with a [`Filesystem`].
pub struct Session<'a> {
  filesystem: &'a Filesystem,

  current_directory: PathBuf,
}

impl<'a> Session<'a> {
  /// Creates a new session.
  #[must_use]
  pub fn new(filesystem: &'a Filesystem) -> Self {
    Self {
      filesystem,
      current_directory: PathBuf::from("/"),
    }
  }

  /// Returns the current path of the session.
  #[must_use]
  pub fn current_directory(&self) -> &Path {
    &self.current_directory
  }

  /// Changes the current working directory.
  ///
  /// # Errors
  ///
  /// This function will return an error if `path` does not exist or isn't a directory.
  pub fn change_directory<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
    let path = path.as_ref().to_owned();

    if !self.resolve(&path)?.is_directory() {
      return Err(Error::NotDirectory(path));
    };

    self.current_directory = path;

    Ok(())
  }

  /// Returns the absolute form of this path.
  fn canonicalize<P: AsRef<Path>>(&self, path: P) -> Result<PathBuf> {
    let path = path.as_ref();

    let path = if path.is_absolute() {
      path.to_owned()
    } else {
      self.current_directory.join(path)
    };

    crate::util::clean_path(path)
  }

  /// Resolves a path to an [`Entry`].
  ///
  /// This is linear in the number of components in the canonicalized `path`.
  ///
  /// # Errors
  ///
  /// This function will return an error if any component of `path` does not exist.
  fn resolve<P: AsRef<Path>>(&self, path: P) -> Result<BorrowedEntry> {
    let path = self.canonicalize(path)?;

    let mut parent = BorrowedEntry::Directory(&self.filesystem.root);

    for component in path.components().skip(1) {
      let Component::Normal(component) = component else {
        return Err(Error::UnsupportedComponent(format!("{component:?}")));
      };

      let BorrowedEntry::Directory(directory) = parent else {
        return Err(Error::NotDirectory(parent.name().into()));
      };

      let Some(next) = directory.entries.get(component) else {
        return Err(Error::NotExist(component.into()));
      };

      parent = BorrowedEntry::from(next);
    }

    Ok(parent)
  }
}
