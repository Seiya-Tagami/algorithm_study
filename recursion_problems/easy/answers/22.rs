fn main() {
  let string_input = "Aburasoba";
  let answer = insert_under_score_at(string_input, 5);
  println!("{}", answer); 
}

fn insert_under_score_at(string_input: &str, index: usize) -> String {
  match string_input {
      text if index < text.len() => {
          return text[..index].to_string() + "_" + &text[index..];
      },
      _ => return string_input.to_string()
  }
}