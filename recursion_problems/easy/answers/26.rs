fn main() {
  let answer = circumference_length(6.0);
  println!("この円の直径は {}", answer); 
}

fn circumference_length(radius: f64) -> f64 {
  return 2.0 * radius * std::f64::consts::PI;
}