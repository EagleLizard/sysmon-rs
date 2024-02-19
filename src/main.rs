
mod sysmon_loop;
mod util;
mod cli_args;

use std::{collections::{HashSet, VecDeque}, fs::{self, canonicalize, DirEntry}, path::{Path, PathBuf}, time::Duration, vec};

use clap::Parser;
use mini_redis::Error;
use sysinfo::{
    Components, Disks, Networks, System,
};
// use sysmon_loop::sysmon_loop::SysmonLoop;
use tokio::time::timeout;
use util::timer::Timer;


use crate::{cli_args::cli_args::{CliArgs, SysmonCli}, sysmon_loop::sysmon_loop::SysmonLoop};

// #[tokio::main]
// async fn main() -> Result<(), Error> {
fn main() {
  // let cli_args = SysmonCli::parse();
  // println!("cli_args:\n{:#?}", cli_args);
  // let sysmon_cli = SysmonCli::parse();
  // println!("sysmon_cli:\n{:#?}", sysmon_cli);
  let SysmonCli::Scandir(scan_dir_args) = SysmonCli::parse();
  println!("scan_dir_args:\n{:#?}", scan_dir_args);
  
  let scan_dir_path = dirs::home_dir().unwrap().join("repos").join("ezd-web");
  let scan_dir_path = PathBuf::new().join(scan_dir_args.dirname);
  println!("Walking dir: {:#?}", scan_dir_path);
  let walk_timer = Timer::start();
  let walk_dir_res = walk_dir(scan_dir_path.as_path());
  let walk_ms = walk_timer.stop();
  println!("files: {}", walk_dir_res.files.len());
  println!("dirs: {}", walk_dir_res.dirs.len());
  println!("Walk took: {:#?}", walk_ms);
  

  // let res = sysmon_loop_test().await;
  // Ok(())
}

struct WalkDirResult {
  files: Vec<PathBuf>,
  dirs: Vec<PathBuf>,
}

fn walk_dir(path: &Path) -> WalkDirResult {
  let root_paths = fs::read_dir(path).unwrap();
  // let mut next_dirs: Vec<DirEntry> = vec![];
  let mut path_queue: VecDeque<PathBuf> = VecDeque::new();
  for path_res in root_paths {
    // let path = path_res.unwrap().path();
    path_queue.push_back(canonicalize(path_res.unwrap().path()).unwrap());
  }

  let mut all_dirs: Vec<PathBuf> = vec![];
  let mut all_files: Vec<PathBuf> = vec![];

  let mut visited_dirs: HashSet<String> = HashSet::new();
  // for root_path in path_queue.clone() {
  //   visited_dirs.insert(root_path.display().to_string());
  // }

  while path_queue.len() > 0 {
    // println!("{}", path_queue.front().unwrap().display());

    // let dir_path = canonicalize(path_queue.pop_front().unwrap()).unwrap();
    let dir_path = path_queue.pop_front().unwrap();
    // if visited_dirs.contains(&dir_path.clone().display().to_string()) {
    //   println!("{}", dir_path.clone().display());
    //   continue;
    // } else {
    //   visited_dirs.insert(dir_path.clone().display().to_string());
    // }
    let is_dir = dir_path.is_dir();
    if is_dir {
      let subdirs = fs::read_dir(dir_path.clone()).unwrap();
      all_dirs.push(dir_path);
      for subdir in subdirs {
        path_queue.push_back(subdir.unwrap().path());
      }
    } else {
      all_files.push(dir_path); 
    }
  }
  // for file_entry in all_files {
  //   println!("{}", file_entry.display());
  // }
  WalkDirResult {
    files: all_files,
    dirs: all_dirs,
  }
}


async fn sysmon_loop_test() -> Result<(), Error> {
  println!("Hello, world!");
  
  sysmon_test();

  let mut sysmon_loop = SysmonLoop::new();

  let unregister_id_1 = sysmon_loop.register(&|loop_count| {
    if (loop_count % 2000) == 0 {
      println!("Registered 1");
    }
  });
  let unregister_id_2 = sysmon_loop.register(&|loop_count| {
    if (loop_count % 5000) == 0 {
      println!("Registered 2");
    }
  });

  // sysmon_loop.register(&|loop_count| {
  //   if (loop_count % 10000) == 0 {
  //     sysmon_test();
  //   }
  // });

  // {
  //   let dereg_timeout_fn = async {
  //     sysmon_loop.unregister(unregister_id_1);
  //   };
  //   let dereg_future = timeout(Duration::from_millis(5000), dereg_timeout_fn);
  //   let res = dereg_future.await;
  // }

  let loop_future = sysmon_loop.run();


  let loop_future_result = match loop_future.await {
    Ok(res) => res,
    Err(error) => panic!("{:?}", error),
  };
  Ok(loop_future_result)
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