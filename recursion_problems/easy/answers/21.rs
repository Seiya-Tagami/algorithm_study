fn main() {
  let string_input = "Aburasoba";
  let answer = last_four_hint(string_input);
  println!("{}", answer); 
}

fn last_four_hint(string_input: &str) -> String {
  match string_input {
      text if text.chars().count() >= 6 => {
          let text_len = text.len();
          let last_four_chars = &text[text_len - 4 .. text_len];
          return "Hint is:".to_string() + last_four_chars;
      },
      _ => "There is no Hint".to_string(),
  }
}