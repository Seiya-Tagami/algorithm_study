fn main() {
  let s1 = "rust is a fantastic language!";
  let s2 = "rust";
  let answer = is_substring(s1, s2);
  println!("{}", answer); 
}

fn is_substring(s1: &str, s2: &str) -> bool {
  return s1.matches(s2).count() > 0;
}