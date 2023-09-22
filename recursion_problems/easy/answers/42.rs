fn main() {
  let result = sheep(12);
  println!("{}", result);
}

fn sheep(count: u32) -> String {
  if count == 0 {
    return "".to_string();
  }

  let str_count = &count.to_string();

  return sheep(count - 1) + str_count + " sheep ~ ";
}