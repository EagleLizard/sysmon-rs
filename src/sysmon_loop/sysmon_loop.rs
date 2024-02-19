use std::{fmt::Error, time::{Duration, SystemTime}};

use tokio::{task::{self, JoinError}, time};


#[derive(Clone)]
pub struct RegisteredEvent<'a> {
  fun: &'a (dyn Fn(u128) -> () + Send + Sync),
  id: u32,
}
#[derive(Clone)]
pub struct SysmonLoop<'a> {
  intervalUs: u64,
  funs: Vec<RegisteredEvent<'a>>,
  id_counter: u32,
}

impl SysmonLoop<'static> {
  pub fn new() -> SysmonLoop<'static> {
    let sysmonLoop = SysmonLoop {
      intervalUs: 500,
      funs: vec![],
      id_counter: 0,
    };
    sysmonLoop
  }
  pub fn register(&mut self, fun: &'static (dyn Fn(u128) -> () + Send + Sync)) -> u32 {
    let curr_id = self.id_counter.clone();
    let registered_event = RegisteredEvent {
      fun,
      id: curr_id,
    };
    self.id_counter = self.id_counter + 1;
    self.funs.push(registered_event);
    // let unregister_fun = move || {
    //   self.unregister(currId)
    // };
    // unregister_fun
    return curr_id;
  }
  pub fn unregister(&mut self, id: u32) {
    let found_idx = self.funs.iter().position(|fun| fun.id == id).unwrap();
    self.funs.remove(found_idx);
  }
  fn exec(&self, loop_count: u128)  {
    self.funs.iter().for_each(|f| {
      (f.fun)(loop_count);
    })
  }
  pub async fn run(self) -> Result<(), JoinError> {
    let start_ms = SystemTime::now();
    let mut loop_count: u128 = 0;
    let mut interval = time::interval(Duration::from_micros(self.intervalUs));
    let loop_fun = async move {
      loop {
        interval.tick().await;
        let elapsed = SystemTime::now().duration_since(start_ms).unwrap();
        if (loop_count % 1000 as u128) == 0 {
          println!("loop_count: {:?}", loop_count);
          println!("elapsed: {:?}", elapsed);
        }
        loop_count = loop_count + 1;
        self.exec(loop_count);
      }
    };
    let loop_run =  task::spawn(loop_fun);
    loop_run.await?
  }
}
