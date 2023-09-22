fn main() {
  let z:u32 = total_candies(10, 2);
  println!("{}", z);
}

fn total_candies(x: u32, y:u32) -> u32 {
  return x * y;
}