fn main() {
  let answer = can_process_order(true, false, false, false, true);
  println!("{}", answer); 
}

fn can_process_order(beef: bool,chicken:bool,salad:bool,coffee:bool,tea:bool) -> bool {
  if beef != chicken && coffee != tea {
      return true;
  }

  return false;
}