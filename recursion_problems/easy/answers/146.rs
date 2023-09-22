fn main() {
  let password = "password";
  let result = password_stars(password);
  println!("{}", result);
}

fn password_stars(password: &str) -> String {
  let mut password_length = password.len();
  let mut result = String::from("");
  while 0 < password_length {
    result += "*";
    password_length -= 1;
  }

  return result;
}