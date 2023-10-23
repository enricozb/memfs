use rand::{distributions::Alphanumeric, thread_rng, Rng};

/// Returns a vector of random ascii characters of a specified size
pub fn random_ascii(n: usize) -> Vec<u8> {
  thread_rng().sample_iter(&Alphanumeric).take(n).collect()
}
