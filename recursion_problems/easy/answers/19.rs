fn main() {
  let email = "test@example.com";
  let answer = is_valid_email(email);
  println!("{}", answer); 
}

fn is_valid_email(email: &str) -> bool {
  let is_not_first_place = email.chars().next() != Some('@');
  let is_not_contained = !email.contains(" ");
  let has_only_one = email.matches("@").count() == 1;
  let is_correct_index = email.find('@') < email.find('.');

  return is_not_first_place && is_not_contained && has_only_one & is_correct_index;
}