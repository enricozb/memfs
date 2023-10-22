use std::path::{Component, Path, PathBuf};

use crate::{Error, Result};

/// Returns `path` with `..` and `.` cleaned.
///
/// Whether or not components are directories is not checked. For example,
/// `clean_path("/a/../b")` will return `"/b"` independent of whether `/a` is
/// a directory.
///
/// # Errors
///
/// This function will return an error if:
/// - `path` is not absolute.
/// - `path` contains any unsupported [`Component`]s.
pub fn clean_path<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
  let path = path.as_ref();

  if !path.is_absolute() {
    return Err(Error::NotAbsolute(path.to_owned()));
  }

  let mut components = Vec::new();

  for component in path.components() {
    match &component {
      Component::RootDir | Component::Normal(_) => components.push(component),
      Component::ParentDir => {
        components.pop().ok_or_else(|| Error::NoParent(path.to_owned()))?;
      }
      Component::CurDir => (),
      Component::Prefix(_) => return Err(Error::UnsupportedComponent(format!("{component:?}"))),
    }
  }

  Ok(components.into_iter().collect())
}
