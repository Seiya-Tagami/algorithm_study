fn main() {
  let answer = is_there_school("Saturday", false);
  println!("{}", answer); 
}

fn is_there_school(day: &str, is_holiday: bool) -> bool {
  match day {
      "Sunday" | "Saturday" => return false,
      _ => {
          if is_holiday {
              return false;
          } else {
              return true
          }
      }
  }
}