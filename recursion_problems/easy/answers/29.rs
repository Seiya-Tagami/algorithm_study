fn main() {
  let answer = vacation_rental(2, 3);
  println!("{}", answer); 
}

fn vacation_rental(people: u32, day:u32) -> f64 {
  let dollar = match day {
      day if day <= 3 => {
          80
      },
      day if day <= 4 => {
          60
      },
      _ => 50
  };

  let per_person = dollar * day;
  let total = (per_person * people) as f64 * 1.12 * 1.08;
  return total.floor();
}