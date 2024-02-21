use std::path::{Path, PathBuf};


const BASE_DIR: &str = env!("CARGO_MANIFEST_DIR");
const DATA_DIR_NAME: &str = "data";

pub fn get_data_dir_path() -> PathBuf {
  let data_dir_path = Path::new(BASE_DIR).join(DATA_DIR_NAME);
  data_dir_path
}
