
use std::time::{Duration, Instant, SystemTime};

use mini_redis::{client, cmd::Unknown, Error};
use sysinfo::{
    Components, Disks, Networks, System,
};
use tokio::{task::{self, JoinError}, time::{self, sleep, timeout, Interval, Sleep}};

#[tokio::main]
async fn main() -> Result<(), Error> {
  println!("Hello, world!");
  
  sysmon_test();

  let mut sysmon_loop = SysmonLoop::new();

  let unregister_id_1 = sysmon_loop.register(&|loop_count| {
    if (loop_count % 200) == 0 {
      println!("Registered 1");
    }
  });
  let unregister_id_2 = sysmon_loop.register(&|loop_count| {
    if (loop_count % 500) == 0 {
      println!("Registered 2");
    }
  });

  // sysmon_loop.exec(0);

  // sysmon_loop.unregister(unregister_id_2);

  // sysmon_loop.exec(0);

  let loop_future = sysmon_loop.run();

  loop_future.await
}

#[derive(Clone)]
pub struct RegisteredEvent<'a> {
  fun: &'a (dyn Fn(u128) -> () + Send + Sync),
  id: u32,
}
#[derive(Clone)]
pub struct SysmonLoop<'a> {
  // interval: Interval,
  funs: Vec<RegisteredEvent<'a>>,
  id_counter: u32,
}

impl SysmonLoop<'static> {
  fn new() -> SysmonLoop<'static> {
    let sysmonLoop = SysmonLoop {
      // interval: time::interval(Duration::from_micros(2000)),
      funs: vec![],
      id_counter: 0,
    };
    sysmonLoop
  }
  fn register(&mut self, fun: &'static (dyn Fn(u128) -> () + Send + Sync)) -> u32 {
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
  fn unregister(&mut self, id: u32) {
    let found_idx = self.funs.iter().position(|fun| fun.id == id).unwrap();
    self.funs.remove(found_idx);
  }
  fn exec(&self, loop_count: u128)  {
    self.funs.iter().for_each(|f| {
      (f.fun)(loop_count);
    })
  }
  async fn run(self) -> Result<(), Error> {
    let start_ms = SystemTime::now();
    let mut loop_count: u128 = 0;
    let mut interval = time::interval(Duration::from_micros(1000));
    let loop_fun = async move {
      loop {
        interval.tick().await;
        let elapsed = SystemTime::now().duration_since(start_ms);
        if (loop_count % 1000) == 0 {
          println!("loop_count: {:?}", loop_count);
          println!("elapsed: {:?}", elapsed.unwrap());
        }
        loop_count = loop_count + 1;
        self.exec(loop_count);
      }
    };
    let loop_run =  task::spawn(loop_fun);
    loop_run.await?
  }
}

async fn evt_loop() -> Result<(), JoinError> {
  let start_ms = SystemTime::now();
  let mut loop_count: u128 = 0;
  let main_loop = task::spawn(async move {
    // let mut interval = time::interval(Duration::from_micros(500));
    let mut interval = time::interval(Duration::from_micros(2000));
    loop {
      interval.tick().await;
      let elapsed = SystemTime::now().duration_since(start_ms);
      if (loop_count % 1000) == 0 {
        println!("loop_count: {:?}", loop_count);
        println!("elapsed: {:?}", elapsed.unwrap());
      }
      loop_count = loop_count + 1;
    }
  });
  main_loop.await
}

fn sysmon_test() {
  // Please note that we use "new_all" to ensure that all list of
  // components, network interfaces, disks and users are already
  // filled!
  let mut sys = System::new_all();

  // First we update all information of our `System` struct.
  sys.refresh_all();

  println!("=> system:");
  // RAM and swap information:
  println!("total memory: {} bytes", sys.total_memory());
  println!("used memory : {} bytes", sys.used_memory());
  println!("total swap  : {} bytes", sys.total_swap());
  println!("used swap   : {} bytes", sys.used_swap());

  // Display system information:
  println!("System name:             {:?}", System::name());
  println!("System kernel version:   {:?}", System::kernel_version());
  println!("System OS version:       {:?}", System::os_version());
  println!("System host name:        {:?}", System::host_name());

  // Number of CPUs:
  println!("NB CPUs: {}", sys.cpus().len());

  // Display processes ID, name na disk usage:
  for (pid, process) in sys.processes() {
      println!("[{pid}] {} {:?}", process.name(), process.disk_usage());
  }

  // We display all disks' information:
  println!("=> disks:");
  let disks = Disks::new_with_refreshed_list();
  for disk in &disks {
      println!("{disk:?}");
  }

  // Network interfaces name, data received and data transmitted:
  let networks = Networks::new_with_refreshed_list();
  println!("=> networks:");
  for (interface_name, data) in &networks {
      println!("{interface_name}: {}/{} B", data.received(), data.transmitted());
  }

  // Components temperature:
  let components = Components::new_with_refreshed_list();
  println!("=> components:");
  for component in &components {
      println!("{component:?}");
  }
}


