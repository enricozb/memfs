use std::borrow::Borrow;

use crate::{Directory, File};

/// An entry in a directory.
pub enum Entry<F = File, D = Directory>
where
  F: Borrow<File>,
  D: Borrow<Directory>,
{
  File(F),
  Directory(D),
}

impl<F, D> Entry<F, D>
where
  F: Borrow<File>,
  D: Borrow<Directory>,
{
  pub fn name(&self) -> &str {
    match self {
      Self::File(file) => &file.borrow().metadata.name,
      Self::Directory(directory) => &directory.borrow().metadata.name,
    }
  }

  /// Returns whether this entry is the [`Self::Directory`] variant.
  pub fn is_directory(&self) -> bool {
    matches!(&self, Self::Directory(_))
  }

  /// Returns whether this entry is the [`Self::File`] variant.
  pub fn is_file(&self) -> bool {
    matches!(&self, Self::File(_))
  }
}

/// An entry with borrowed inner structures.
pub type Borrowed<'a> = Entry<&'a File, &'a Directory>;

impl<'a> From<&'a Entry> for Borrowed<'a> {
  fn from(entry: &'a Entry) -> Self {
    match entry {
      Entry::Directory(directory) => Self::Directory(directory),
      Entry::File(file) => Self::File(file),
    }
  }
}
