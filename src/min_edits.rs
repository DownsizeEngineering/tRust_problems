pub fn run() {
  println!("Hello from min_edits");
  min_edits("banana", "phone");
}

fn min_edits(a: &str, b: &str) -> u64 {
  println!("{} {}", a, b);
  0
}

#[cfg(test)]
mod tests {
  #[test]
  fn two_plus_two() {
    assert_eq!(2 + 2, 4);
  }
}