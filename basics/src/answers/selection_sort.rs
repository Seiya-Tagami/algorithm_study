use rand::{self, Rng};

fn main() {
    let mut rng = rand::thread_rng();
    let mut arr: Vec<i32> = (0..10).map(|_| rng.gen_range(0..100)).collect();
    println!("{:?}", arr);

    selection_sort(&mut arr);

    println!("selection sort\n{:?}", arr);
}

fn selection_sort(a: &mut Vec<i32>) {
    let n = a.len();
    for i in 0..n {
        let mut min = i;
        // 仮定した最小の添字の次の添字から
        for j in i + 1..n {
            if a[j] < a[min] {
                min = j;
            }
        }
        a.swap(i, min)
    }
}

// 参考：https://ja.wikipedia.org/wiki/%E9%81%B8%E6%8A%9E%E3%82%BD%E3%83%BC%E3%83%88
// 参考：https://www.youtube.com/watch?v=aC2kGoHY3_Q