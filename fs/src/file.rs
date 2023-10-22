use crate::metadata::Metadata;

/// A file.
pub struct File {
  pub metadata: Metadata,

  content: Vec<u8>,
}
