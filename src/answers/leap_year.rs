fn main() {
  let years_to_check = vec![2000, 2004, 2100, 2400];

  for year in years_to_check {
      if is_leap_year(year) {
          println!("{:04}年は閏年", year);
      } else {
          println!("{:04}年は閏年ではない", year)
      }
  }
}

fn is_leap_year(year: i32) -> bool {
  match (year % 4, year % 100, year % 400) {
      (_, _, 0) => true,
      (_, 0, _) => false,
      (0, _, _) => true,
      _ => false
  }
}
