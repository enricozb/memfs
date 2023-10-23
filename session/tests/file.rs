use std::path::PathBuf;

use fs::Filesystem;
use session::*;

#[test]
fn create_file() {
  let mut session = Session::new(Filesystem::new());
  session.create_file("/a").unwrap();

  let (_, entry) = session.resolve("/a").unwrap();

  assert!(entry.is_file());
}

#[test]
fn create_file_parent_not_exist() {
  let mut session = Session::new(Filesystem::new());

  let err = session.create_file("/a/b").err().unwrap();

  assert!(matches!(err, Error::NotExist(path) if path == PathBuf::from("a")));
}

#[test]
fn create_file_already_exists() {
  let mut session = Session::new(Filesystem::new());
  session.create_file("/a").unwrap();

  let (_, entry) = session.resolve("/a").unwrap();

  assert!(entry.is_file());

  let res = session.create_file("/a");

  assert!(matches!(res, Err(Error::Exists(_))));
}
