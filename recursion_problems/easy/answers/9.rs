fn main() {
  let string_output:String = name_initials("Moka", "Aoba");
  println!("{}", string_output);
}

fn name_initials(first_name: &str, last_name: &str) -> String {
  let first_name_initial:char = first_name.chars().next().unwrap();
  let last_name_initial:char = last_name.chars().next().unwrap();
  return first_name_initial.to_string() + "." +  &last_name_initial.to_string();
}