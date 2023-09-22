fn main() {
  let answer = how_much_is_your_debt(10.0);
  println!("{}", answer);
}

fn how_much_is_your_debt(year: f64) -> f64 {
  let interest:f64 = 1.2;
  let total = 10000.0 * interest.powf(year);
  return total.floor();
} 