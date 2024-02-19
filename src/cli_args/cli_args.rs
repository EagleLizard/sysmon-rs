
use clap::Parser;

#[derive(Parser, Debug)]
pub enum SysmonCli {
  Scandir(ScanDirArgs),
}

#[derive(clap::Args, Debug)]
#[command(version, about)]
pub struct ScanDirArgs {
  pub dirname: String,
}

#[derive(Parser, Debug)]
pub struct CliArgs {
  #[arg(short, long)]
  pub scan_dir: Option<String>,
}