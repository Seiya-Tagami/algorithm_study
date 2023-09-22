fn main() {
  let result = total_square_area(12);
  println!("{}", result);
}

fn total_square_area(x: u32) -> u32 {
  if x == 0 {
    return 0;
  }

  return total_square_area(x - 1) + x.pow(3);
}