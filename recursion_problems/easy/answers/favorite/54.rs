// FizzBuzz問題
fn main() {
  let n: usize= 20;
  println!("FizzBuzz");
  fizz_buzz(n);
}

fn fizz_buzz(n:usize) {
  for i in 1..n {
      match (i % 3, i % 5) {
          (0, 0) => println!("{} : FizzBuzz", i),
          (0, _) => println!("{} : Fizz", i),
          (_, 0) => println!("{} : Buzz", i),
          _ => println!("{}", i)
      }
  }
}