use std::{
  borrow::Borrow,
  ffi::{OsStr, OsString},
};

use crate::{metadata::Metadata, Directory, File};

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
  pub fn name(&self) -> &OsStr {
    match self {
      Self::File(file) => &file.borrow().metadata.name,
      Self::Directory(directory) => &directory.borrow().metadata.name,
    }
  }

  pub fn metadata(&self) -> &Metadata {
    match self {
      Self::File(file) => &file.borrow().metadata,
      Self::Directory(directory) => &directory.borrow().metadata,
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

impl Entry<File, Directory> {
  pub fn rename(&mut self, name: OsString) {
    match self {
      Self::File(file) => file.metadata.name = name,
      Self::Directory(directory) => directory.metadata.name = name,
    }
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

/// An entry with mutably borrowed inner structures.
pub type MutBorrowed<'a> = Entry<&'a mut File, &'a mut Directory>;

impl<'a> From<&'a mut Entry> for MutBorrowed<'a> {
  fn from(entry: &'a mut Entry) -> Self {
    match entry {
      Entry::Directory(directory) => Self::Directory(directory),
      Entry::File(file) => Self::File(file),
    }
  }
}
