pub mod error;
pub mod util;

use std::{
  collections::btree_map::Entry as BTreeMapEntry,
  path::{Component, Path, PathBuf},
};

use fs::{
  entry::{Borrowed as BorrowedEntry, MutBorrowed as MutBorrowedEntry},
  Directory, Entry, Filesystem,
};

pub use self::error::{Error, Result};

/// An interactive session with a [`Filesystem`].
pub struct Session {
  filesystem: Filesystem,

  current_directory: PathBuf,
}

impl Session {
  /// Creates a new session.
  #[must_use]
  pub fn new(filesystem: Filesystem) -> Self {
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
    let (path, entry) = self.resolve(&path)?;

    if !entry.is_directory() {
      return Err(Error::NotDirectory(path));
    };

    self.current_directory = path;

    Ok(())
  }

  /// Creates a new directory.
  ///
  /// # Errors
  ///
  /// This function will return an error if:
  /// - an entry already exists with this name.
  /// - the parent of `path` does not exist or is not a directory.
  pub fn create_directory<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
    let path = self.canonicalize(path.as_ref())?;

    let Some(parent) = path.parent() else {
      return Err(Error::NoParent(path));
    };

    let (_, entry) = self.resolve_mut(parent)?;
    let MutBorrowedEntry::Directory(directory) = entry else {
      return Err(Error::NotDirectory(path));
    };

    let Some(name) = path.file_name() else {
      return Err(Error::NoFileName(path));
    };

    match directory.entries.entry(name.to_owned()) {
      BTreeMapEntry::Occupied(_) => return Err(Error::Exists(name.to_owned())),
      BTreeMapEntry::Vacant(v) => v.insert(Entry::Directory(Directory::new(name))),
    };

    Ok(())
  }

  /// Returns the entries of `path`.
  ///
  /// # Errors
  ///
  /// This function will return an error if `path` does not exist or isn't a directory.
  pub fn list_directory<P: AsRef<Path>>(&self, path: P) -> Result<impl Iterator<Item = &Entry>> {
    let (path, entry) = self.resolve(path)?;

    let BorrowedEntry::Directory(directory) = entry else {
      return Err(Error::NotDirectory(path));
    };

    Ok(directory.entries.values())
  }

  /// Removes a directory or file.
  ///
  /// # Errors
  ///
  /// This function will return an error if:
  /// - the path does not exist.
  /// - the path does not have a parent (`/`).
  pub fn remove<S: AsRef<Path>>(&mut self, path: S) -> Result<()> {
    let path = self.canonicalize(path.as_ref())?;

    let Some(parent) = path.parent() else {
      return Err(Error::NoParent(path));
    };

    let (_, entry) = self.resolve_mut(parent)?;
    let MutBorrowedEntry::Directory(directory) = entry else {
      return Err(Error::NotDirectory(path));
    };

    let Some(name) = path.file_name() else {
      return Err(Error::NoFileName(path));
    };

    if directory.entries.remove(name).is_none() {
      return Err(Error::NotExist(name.into()));
    }

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
  fn resolve<P: AsRef<Path>>(&self, path: P) -> Result<(PathBuf, BorrowedEntry)> {
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

    Ok((path, parent))
  }

  /// Resolves a path to a mutable [`Entry`].
  ///
  /// This is linear in the number of components in the canonicalized `path`.
  ///
  /// # Errors
  ///
  /// This function will return an error if any component of `path` does not exist.
  fn resolve_mut<P: AsRef<Path>>(&mut self, path: P) -> Result<(PathBuf, MutBorrowedEntry)> {
    let path = self.canonicalize(path)?;

    let mut parent = MutBorrowedEntry::Directory(&mut self.filesystem.root);

    for component in path.components().skip(1) {
      let Component::Normal(component) = component else {
        return Err(Error::UnsupportedComponent(format!("{component:?}")));
      };

      let MutBorrowedEntry::Directory(directory) = parent else {
        return Err(Error::NotDirectory(parent.name().into()));
      };

      let Some(next) = directory.entries.get_mut(component) else {
        return Err(Error::NotExist(component.into()));
      };

      parent = MutBorrowedEntry::from(next);
    }

    Ok((path, parent))
  }
}
