fn main() {
  let string_output:char = string_first("Hello World");
  println!("{}", string_output);
}

fn string_first(string_input: &str) -> char {
  let ch1:char = string_input.chars().next().unwrap();
  return ch1;
}