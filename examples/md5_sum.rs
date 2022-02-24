use md5;
use std::time::Instant;
fn main() {
  let start = Instant::now();
  let mut bytes = String::from("abcdefghijklmnopqrstuvwxyz");
  for _ in 0..1000000 {
    let digest = md5::compute(bytes);
    bytes = format!("{:x}", digest);
  }
  let duration = start.elapsed();
  println!("duration: {}", duration.as_secs_f32());
  println!("{:?}", bytes);
}