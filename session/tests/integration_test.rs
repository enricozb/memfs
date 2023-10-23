use std::path::PathBuf;

use fs::Filesystem;
use session::*;

#[test]
fn change_directory() {
  let mut session = Session::new(Filesystem::new());
  session.create_directory("/a").unwrap();

  let res = session.change_directory("/a");

  assert!(res.is_ok());
}

#[test]
fn change_directory_not_exist() {
  let mut session = Session::new(Filesystem::new());
  let res = session.change_directory("/a/b/c");

  assert!(matches!(res, Err(Error::NotExist(_))));
}

#[test]
fn change_directory_not_directory() {
  let mut session = Session::new(Filesystem::new());
  session.create_file("/a").unwrap();

  let res = session.change_directory("/a");

  assert!(matches!(res, Err(Error::NotDirectory(_))));
}

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
