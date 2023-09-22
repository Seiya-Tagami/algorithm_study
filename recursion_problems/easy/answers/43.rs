fn main() {
  let result = reverse_string("clean_architecture");
  println!("{}", result);
}

fn reverse_string(string_input: &str) -> String {
  if string_input.len() == 0 {
    return "".to_string();
  }

  return string_input.chars().last().unwrap().to_string() + &reverse_string(&string_input[..string_input.len() - 1]);
}