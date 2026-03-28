use base64::Engine;
use rand::{TryRng, rngs::SysRng};

pub fn execute(prefix: &str) -> String {
  let mut bytes = [0u8; 32];
  SysRng.try_fill_bytes(&mut bytes).unwrap();

  format!(
    "{}{}",
    prefix,
    base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes)
  )
}
