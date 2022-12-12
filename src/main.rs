fn generate() -> u8 {
  use rand::Rng;
  let mut rng = rand::thread_rng();
  let num: u8 = rng.gen_range(0..10);
  return num;
}

fn main() {
    println!("{}",generate());
}