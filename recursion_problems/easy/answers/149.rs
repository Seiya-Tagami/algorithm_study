fn main() {
  let v: Vec<String> = vec![
    String::from("aaa"),
    String::from("bbb"),
    String::from("ccc"),
  ];
  let result = join_words(v, "-");
  println!("{}", result);
}

fn join_words(string_vec: Vec<String>, delimiter: &str) -> String {
  let s: String = string_vec.join(delimiter);
  return s
}