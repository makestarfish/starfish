use rand::RngExt;
use std::iter::repeat_with;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

pub fn execute() -> String {
  repeat_with(|| CHARSET[rand::rng().random_range(0..CHARSET.len())] as char)
    .take(10)
    .collect()
}
