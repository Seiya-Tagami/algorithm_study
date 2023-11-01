fn main() {
  let answer = divisor(12, 12);
  println!("{}", answer); 
}

fn divisor(n: i32, x: i32) -> String {
  if x == 1 {
      return "1".to_string();
  } else if n % x == 0 {
      return format!("{}-{}", divisor(n, x - 1), x);
  } else {
      return divisor(n, x - 1);
  }
}