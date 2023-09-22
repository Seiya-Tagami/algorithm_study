fn main() {
  let answer = get_bootstrap_class(1000);
  println!("{}", answer); 
}

fn get_bootstrap_class(screen_width: u32) -> &'static str {
  match screen_width {
      width if width >= 1200 => "lg",
      width if width >= 992 => "md",
      width if width >= 768 => "sm",
      _ => "xs",
  }
}