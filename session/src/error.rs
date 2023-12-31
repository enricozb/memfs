use std::{ffi::OsString, path::PathBuf};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
  #[error("{0:?} is not a directory")]
  NotDirectory(PathBuf),

  #[error("{0:?} is not a file")]
  NotFile(PathBuf),

  #[error("{0:?} does not exist")]
  NotExist(PathBuf),

  #[error("{0:?} is not absolute")]
  NotAbsolute(PathBuf),

  #[error("{0:?} has no parent")]
  NoParent(PathBuf),

  #[error("{0:?} has no file name")]
  NoFileName(PathBuf),

  #[error("{0:?} already exists")]
  Exists(OsString),

  #[error("unsupported component {0}")]
  UnsupportedComponent(String),
}
