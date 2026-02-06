use base64::Engine;
use rand::{TryRngCore, rngs::OsRng};

pub fn execute(prefix: &str) -> String {
  let mut bytes = [0u8; 32];
  OsRng.try_fill_bytes(&mut bytes).unwrap();

  format!(
    "{}{}",
    prefix,
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
  )
}
