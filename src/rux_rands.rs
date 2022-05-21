use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use crate::rux_debug::{dbg_call, dbg_reav};

pub fn range(min: u32, max: u32) -> u32 {
  dbg_call!(min, max);
  thread_rng().gen_range(min..max)
}

pub fn chars(count: usize) -> String {
  dbg_call!(count);
  dbg_reav!(thread_rng()
    .sample_iter(&Alphanumeric)
    .take(count)
    .map(char::from)
    .collect());
}

pub fn lines(lines_count: usize, chars_count: usize) -> Vec<String> {
  dbg_call!(lines_count, chars_count);
  let mut lines = Vec::with_capacity(lines_count);
  for _ in 0..lines_count {
    lines.push(chars(chars_count));
  }
  dbg_reav!(lines);
}
