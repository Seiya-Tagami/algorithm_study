fn main() {
    let answer = is_rational_number(5.0);
    println!("{}", answer); 
}

fn is_rational_number(number: f64) -> bool {
    return number.sqrt() % 1.0 == 0.0;
}