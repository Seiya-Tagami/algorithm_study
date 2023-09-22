fn main() {
  let y:u32 = cube_surface_area(8);
  println!("{}", y);
}

fn cube_surface_area(x: u32) -> u32 {
  return x.pow(2) * 6;
}