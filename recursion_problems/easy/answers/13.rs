fn main() {
  let answer = my_xor(45, 45);
  println!("{}", answer); 
}

fn my_xor(x: u32, y:u32) -> bool {
  let is_same: bool = x == y;
  return is_same;
}