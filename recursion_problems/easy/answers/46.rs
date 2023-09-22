fn main() {
  let result = divide_by_3_count(12);
  println!("{}", result);
}

fn divide_by_3_count(n: u32) -> u32 {
  if n == 1 {
    return 0;
  }

  return divide_by_3_count(n / 3) + 1;
}