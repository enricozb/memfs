/// A struct describing what to do on an entry during walking.
///
/// See [`Session::walk`].
///
/// [`Session::walk`]: crate::session::Session::walk
pub enum Walk {
  Skip,
  Continue,
}

impl From<()> for Walk {
  fn from((): ()) -> Self {
    Self::Continue
  }
}
