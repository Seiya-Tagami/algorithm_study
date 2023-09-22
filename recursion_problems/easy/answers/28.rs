fn main() {
  let answer = is_leap_year(2003);
  println!("{}", answer);
}

fn is_leap_year(year: u32) -> bool {
  match year {
    y if y % 100 == 0 => {
      if y % 400 == 0 {
          return true
      }
      return false
    }
    y if y % 4 == 0 => return true,
    _ => return false
  }
}