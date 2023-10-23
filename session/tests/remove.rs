use fs::Filesystem;
use session::*;

#[test]
fn remove_directory() {
  let mut session = Session::new(Filesystem::new());
  session.create_directory("/a").unwrap();
  session.resolve("/a").unwrap();
  session.remove("/a").unwrap();

  let res = session.resolve("/a");

  assert!(matches!(res, Err(Error::NotExist(_))));
}

#[test]
fn remove_file() {
  let mut session = Session::new(Filesystem::new());
  session.create_file("/a").unwrap();
  session.resolve("/a").unwrap();
  session.remove("/a").unwrap();

  let res = session.resolve("/a");

  assert!(matches!(res, Err(Error::NotExist(_))));
}

#[test]
fn remove_not_exist() {
  let mut session = Session::new(Filesystem::new());

  let res = session.remove("/a");

  assert!(matches!(res, Err(Error::NotExist(_))));
}
