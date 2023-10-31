// フィボナッチ数列
fn main() {
  let answer = fibonacci(0, 1, 9);
  println!("{}", answer); 
}

fn fibonacci(fn1:i32, fn2:i32, n: i32) -> i32 {
  if n == 0 {
      return fn1;
  }

  let x = fn2;
  let y = fn1 + fn2;

  return fibonacci(x, y, n - 1);
}