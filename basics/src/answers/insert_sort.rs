fn main() {
  let mut arr = vec![5, 2, 1, 4, 3];
  insert_sort(&mut arr);
  println!("{:?}", arr);
}

fn insert_sort(a: &mut [i32]) {
  for i in 1..a.len() {
      // 要素を一つずつ追加して、処理を行う
      insert(&mut a[0..=i])
  }
}

fn insert(a: &mut [i32]) {
  for i in (1..a.len()).rev() {
      if a[i - 1] > a[i] {
          a.swap(i - 1, i)
      } else {
          break;
      }
  }
}