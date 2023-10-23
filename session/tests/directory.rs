use std::path::PathBuf;

use fs::Filesystem;
use session::*;

#[test]
fn create_directory() {
  let mut session = Session::new(Filesystem::new());
  session.create_directory("/a").unwrap();

  let (_, entry) = session.resolve("/a").unwrap();

  assert!(entry.is_directory());
}

#[test]
fn create_directory_parent_not_exist() {
  let mut session = Session::new(Filesystem::new());

  let err = session.create_directory("/a/b").err().unwrap();

  assert!(matches!(err, Error::NotExist(path) if path == PathBuf::from("a")));
}

#[test]
fn create_directory_already_exists() {
  let mut session = Session::new(Filesystem::new());
  session.create_directory("/a").unwrap();

  let (_, entry) = session.resolve("/a").unwrap();

  assert!(entry.is_directory());

  let res = session.create_directory("/a");

  assert!(matches!(res, Err(Error::Exists(_))));
}
