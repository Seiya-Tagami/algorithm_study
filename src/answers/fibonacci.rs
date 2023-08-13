fn main() {
  println!("{}", fibonacci(0, 1, 10))
}

fn fibonacci(fn1: i32, fn2: i32, n: i32) -> i32 {
  if n == 0 {
      return fn1
  }
  return fibonacci(fn2, fn1 + fn2, n - 1);
}