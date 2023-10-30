fn main() {
  let x: u32 = 28;
  let y: u32 = 32;

  let square = gcp(x, y).pow(2);
  let cnt = (x * y) / square;
  println!("{}", cnt); 
}

// ユークリッドの互除法で最大公約数を求める
fn gcp(x: u32, y: u32) -> u32 {
  if x % y == 0 {
      return y;
  }
  return gcp(y, x%y)
}