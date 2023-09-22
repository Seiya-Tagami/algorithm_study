fn main() {
  let result = number_of_dots(1000);
  println!("{}", result);
}

fn number_of_dots(x:u32) -> u32 {
  if x == 0 {
    return 0;
  }
  return number_of_dots(x - 1) + x;
}