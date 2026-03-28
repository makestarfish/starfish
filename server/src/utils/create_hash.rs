use sha2::{Digest, Sha224};

pub fn execute(value: &str) -> String {
  hex::encode(&Sha224::digest(value)[..])
}
