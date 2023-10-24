use fs::{Entry, Filesystem};
use session::*;

#[test]
fn move_directory() {
  let mut session = Session::new(Filesystem::new());
  session.create_directory("/a").unwrap();
  session.create_file("/a/1").unwrap();
  session.move_entry("/a", "/b").unwrap();

  let res = session.resolve("/a");
  assert!(matches!(res, Err(Error::NotExist(_))));

  let entries: Vec<_> = session.list_directory("/b").unwrap().map(Entry::name).collect();
  assert_eq!(entries, &["1"]);
}

#[test]
fn move_file() {
  const CONTENT: &str = "some text";

  let mut session = Session::new(Filesystem::new());
  session.create_file("/a").unwrap();
  session.write_file("/a", CONTENT.into()).unwrap();
  session.move_entry("/a", "/b").unwrap();

  let res = session.resolve("/a");
  assert!(matches!(res, Err(Error::NotExist(_))));

  let content = session.read_file("/b").unwrap();
  assert_eq!(content, CONTENT);
}
