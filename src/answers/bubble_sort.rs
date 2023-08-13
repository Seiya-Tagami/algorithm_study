use rand::{self, Rng};

fn main() {
    let mut rng = rand::thread_rng();
    let mut arr: Vec<i32> = (0..10).map(|_| rng.gen_range(0..100)).collect();
    println!("{:?}", arr);

    let (cmp, swap) = bubble_sort(&mut arr);

    println!("bubble sort\n{:?}", arr);
    println!("tmp = {}\nswap = {}", cmp, swap)
}

fn bubble_sort(a: &mut Vec<i32>) -> (i32, i32) {
    let mut cmp_count: i32 = 0;
    let mut swap_count: i32 = 0;

    let n = a.len();
    for i in 0..n - 1 { 
        let mut swapped = false;
        // 最適化１、n - i - 1で再び同じ数を比較しないようにする
        for j in 0..n - i - 1 {
            if a[j] > a[j + 1] {
                a.swap(j, j + 1);
                swapped = true;
                swap_count += 1;
            }
            cmp_count += 1;
        }

        // 最適化２、一度も交換がなされない場合は既にソート済みのものとみなす
        if !swapped {
            break;
        }
    }

    (cmp_count, swap_count)
}

// 参考：https://ja.wikipedia.org/wiki/%E3%83%90%E3%83%96%E3%83%AB%E3%82%BD%E3%83%BC%E3%83%88https://ja.wikipedia.org/wiki/%E3%83%90%E3%83%96%E3%83%AB%E3%82%BD%E3%83%BC%E3%83%88
// 参考：https://www.youtube.com/watch?v=69wrkarV0IQ