fn main() {
  let answer: i64 = split_and_add(18);
  println!("{}", answer);
}

fn split_and_add(digits: i64) -> i64 {
  if digits < 10 {
      return digits;
  }
  return (digits % 10) + split_and_add(digits / 10);
}
