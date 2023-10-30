fn main() {
  let answer: f64 = split_and_add(18.0);
  println!("{}", answer);
}

fn split_and_add(digits: f64) -> f64 {
  if digits < 10.0 {
      return digits;
  }
  return (digits % 10.0) + split_and_add((digits / 10.0).floor())
}