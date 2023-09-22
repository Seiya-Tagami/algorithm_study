fn main() {
  let string_output:char = string_last("Hello World");
  println!("{}", string_output);
}

fn string_last(string_input: &str) -> char {
  let ch_last:char = string_input.chars().last().unwrap();
  return ch_last;
}