/// Calculates the platform fee (4% + 40¢) given the transaction amount
pub fn execute(amount: i64) -> i64 {
  (amount / 100 * 4) + 40
}

#[cfg(test)]
pub mod tests {
  use super::execute as calculate_platform_fee;

  #[test]
  fn example() {
    assert_eq!(calculate_platform_fee(9600), 424)
  }
}
