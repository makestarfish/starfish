pub fn execute(email: &str) -> String {
  format!(
    "https://www.gravatar.com/avatar/{:x}?d=404",
    md5::compute(email.as_bytes())
  )
}
