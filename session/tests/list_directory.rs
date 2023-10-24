use fs::{Entry, Filesystem};
use session::*;

#[test]
fn list_directory_root() {
  let mut session = Session::new(Filesystem::new());
  session.create_directory("/a").unwrap();
  session.create_file("/a/1.txt").unwrap();
  session.create_file("/b").unwrap();

  let entries: Vec<_> = session.list_directory("/").unwrap().map(Entry::name).collect();

  assert_eq!(entries, &["a", "b"]);
}

#[test]
fn list_directory_subdirectory() {
  let mut session = Session::new(Filesystem::new());
  session.create_directory("/a").unwrap();
  session.create_file("/a/1.txt").unwrap();
  session.create_file("/b").unwrap();

  let entries: Vec<_> = session.list_directory("/a").unwrap().map(Entry::name).collect();

  assert_eq!(entries, &["1.txt"]);
}

#[test]
fn list_directory_not_directory() {
  let mut session = Session::new(Filesystem::new());
  session.create_file("/a").unwrap();

  let res = session.list_directory("/a");

  assert!(matches!(res, Err(Error::NotDirectory(_))));
}
