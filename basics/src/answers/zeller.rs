fn main() {
  let year = 2023;
  let month = 8;
  let day = 15;

  let zeller_iso = zeller(year, month, day);
  println!("{:04}-{:02}-{:02} ({})", year, month, day, zeller_iso)
}

fn zeller(y: i32, m:i32, d:i32) -> i32 {
  let century = y / 100;
  let gamma = if (4..1582).contains(&y) {
      6 * century + 5 // ユリウス歴
  } else {
      5 * century + century / 4 // グレゴリオ歴
  };
  
  let (year, month) = if m <= 2 {
      (y - 1, m + 12)
  } else {
      (y, m)
  };
  let year_12 = year % 100;

  // ツェラーの公式で計算
  (d + (26 * (month + 1)) / 10 + year_12 + (year_12 / 4) + gamma + 5) % 7 + 1
}

// 参考: https://www.youtube.com/watch?v=PtuwXqy2LXg