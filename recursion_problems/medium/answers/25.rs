// 文字列の真ん中を返す
fn main() {
  let string_input = "ABCDEF";
  let answer = middle_substring(string_input);
  println!("{}", answer); 
}

// マルチバイト文字は対応していないので注意
fn middle_substring(string_input: &str) -> &str {
  let length = string_input.len();
  let middle = (length as f64 / 2.0).floor() as usize;
  let front = (middle as f64 / 2.0).ceil() as usize;

  if length <= 2 {
      return &string_input[..1];
  }

  return &string_input[front..(front + middle)]
}