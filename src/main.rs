
mod sysmon_loop;
mod util;
mod cli_args;

use std::{collections::{HashSet, VecDeque}, fs::{self, canonicalize, symlink_metadata, DirEntry}, io, path::{Path, PathBuf}, time::Duration, vec};

use clap::Parser;
use mini_redis::Error;
use same_file::is_same_file;
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
  let walk_dir_res = walk_dir(&scan_dir_path);
  let walk_ms = walk_timer.stop();
  
  // for dir in walk_dir_res.dirs {
  //   println!("{}", dir.display());
  // }
  // for file in walk_dir_res.files.clone() {
  //   println!("{}", file.display());
  // }

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

fn walk_dir(path: &PathBuf) -> WalkDirResult {
  // let root_paths = fs::read_dir(path).unwrap();
  // let mut next_dirs: Vec<DirEntry> = vec![];
  let mut path_queue: VecDeque<PathBuf> = VecDeque::new();
  // for path_res in root_paths {
  //   // let path = path_res.unwrap().path();
  //   // path_queue.push_back(canonicalize(path_res.unwrap().path()).unwrap());
  //   path_queue.push_back(path_res.unwrap().path());
  // }
  path_queue.push_back(path.to_path_buf());

  let mut all_dirs: Vec<PathBuf> = vec![];
  let mut all_files: Vec<PathBuf> = vec![];

  while path_queue.len() > 0 {
    let dir_path = path_queue.pop_front().unwrap();
    let is_dir = dir_path.is_dir();
    if is_dir {
      let meta = symlink_metadata(dir_path.as_path()).unwrap();
      let mut is_loop = false;
      if meta.is_symlink() {
        println!("symlink: {}", dir_path.display());
        is_loop = true;
      }
      // let is_loop = contains_loop(dir_path.clone());
      if !is_loop {
        let subdirs = fs::read_dir(dir_path.clone()).unwrap();
        all_dirs.push(dir_path);
        for subdir_res in subdirs {
          let subdir = subdir_res.unwrap();
          path_queue.push_back(subdir.path());
        }
      };
    } else {
      all_files.push(dir_path); 
    }
  }

  WalkDirResult {
    files: all_files,
    dirs: all_dirs,
  }
}

fn contains_loop<P: AsRef<Path>>(path: P) -> bool {
  let path = path.as_ref();
  let mut path_buf = path.to_path_buf();
  while path_buf.pop() {
    let same_dir = is_same_file(&path_buf, path).unwrap();
    if same_dir {
      return true;
    }
  }
  false
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