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
