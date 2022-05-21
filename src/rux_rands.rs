use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn range(min: u32, max: u32) -> u32 {
  thread_rng().gen_range(min..max)
}

pub fn chars(count: usize) -> String {
  thread_rng()
    .sample_iter(&Alphanumeric)
    .take(count)
    .map(char::from)
    .collect()
}
