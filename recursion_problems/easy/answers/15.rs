fn main() {
  let answer = screen_view_mode(45, 60);
  println!("{}", answer); 
}

fn screen_view_mode(height: u32, width:u32) -> &'static str {
  if height >= width {
      return "portrait"
  } else {
      return "landscape"
  }
}