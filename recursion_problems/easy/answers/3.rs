fn main() {
  let y:u32 = cube_volume(10);
  println!("{}", y);
}

fn cube_volume(x: u32) -> u32 {
  return x.pow(3);
}