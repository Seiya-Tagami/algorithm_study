fn main() {
  let mil:f64 = meters_to_miles(1000.0);
  println!("{}", mil);
}

fn meters_to_miles(meters: f64) -> f64 {
  let mile_per_one_meter: f64 = 0.000621371;
  return meters * mile_per_one_meter;
}