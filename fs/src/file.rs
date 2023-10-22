use crate::metadata::Metadata;

/// A file.
pub struct File {
  metadata: Metadata,

  content: Vec<u8>,
}
