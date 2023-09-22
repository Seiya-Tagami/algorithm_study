fn main() {
  let answer = weekly_7days_sales(450);
  println!("{}", answer); 
}

fn weekly_7days_sales(ticket_price: u32) -> u32 {
  let per_person_price = ticket_price - 250;
  return 150000 - (700 * per_person_price);
}