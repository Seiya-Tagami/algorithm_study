fn main() {
  let z:u32 = get_lowest_temperature(10, 2);
  println!("{}", z);
}

fn get_lowest_temperature(x: u32, y:u32) -> u32 {
  return x - y;
}