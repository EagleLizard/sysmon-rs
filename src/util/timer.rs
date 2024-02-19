
use std::time::{Duration, Instant};

pub fn run_and_time<F>(mut fun: F) -> Duration
where
  F: FnMut()
{
  let start = Instant::now();
  fun();
  let fun_time = start.elapsed();
  return fun_time;
}

pub struct Timer {
  start_time: Instant,
}
impl Timer {
  fn new() -> Timer {
    Timer {
      start_time: Instant::now(),
    }
  }
  pub fn start() -> Timer {
    Timer::new()
  }
  pub fn stop(&self) -> Duration {
    self.start_time.elapsed()
  }
}
