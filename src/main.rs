//! dotenv-exec cli
//! ==============
//!

use std::fs;
use std::io;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::{exit, Command};

use dotenv;
use structopt::StructOpt;
use users::{get_group_by_name, get_user_by_name};

#[derive(Debug, StructOpt)]
#[structopt(
  name = "dotenv-exec",
  about = "Execute a command expanding specified dotenv files in the process environment."
)]
struct Options {
  /// User to run the command as
  #[structopt(short, long)]
  user: Option<String>,

  /// Group to run the command as
  #[structopt(short, long)]
  group: Option<String>,

  /// Working directory to run the command in
  #[structopt(short, long)]
  directory: Option<String>,

  /// .env file(s) to load
  #[structopt(short = "f", long = "file", multiple = true)]
  files: Vec<PathBuf>,

  /// Ignore any missing env files
  #[structopt(long)]
  ignore_missing: bool,

  /// Disable default .env lookup
  #[structopt(long)]
  no_default: bool,

  /// Program to execute
  cmd: String,

  /// Program arguments and options
  args: Vec<String>,
}

fn main() {
  let options = Options::from_args();

  let mut command = Command::new(options.cmd);

  command.args(options.args);

  set_directory(&mut command, options.directory);
  set_user(&mut command, options.user);
  set_group(&mut command, options.group);

  if !options.no_default {
    match dotenv::dotenv() {
      Ok(_) => {}
      Err(e) => {
        eprintln!("Failed to load default env file: {}", e);
        exit(1);
      }
    }
  }

  for path in options.files.iter().map(|pb| pb.as_path()) {
    match dotenv::from_path(path) {
      Ok(_) => {}
      Err(dotenv::Error::Io(e)) => {
        dbg!(&e);
        if e.kind() != io::ErrorKind::NotFound || !options.ignore_missing {
          eprintln!("Failed to load env file at {}: {}", path.display(), e);
          exit(1);
        }
      }
      Err(e) => {
        eprintln!("Failed to load env file at {}: {}", path.display(), e);
        exit(1);
      }
    }
  }

  // exec doesn't return unless something went wrong.
  let err = command.exec();
  eprintln!("Failed to exec: {}", err);
  exit(1);
}

fn set_directory(command: &mut Command, directory: Option<String>) {
  if let Some(d) = directory {
    if match fs::metadata(&d) {
      Ok(m) => m.is_dir(),
      _ => false,
    } {
      command.current_dir(d);
    } else {
      eprintln!("{} is not a directory", d);
      exit(1);
    }
  }
}

fn set_user(command: &mut Command, user: Option<String>) {
  if let Some(u) = user {
    match get_user_by_name(&u) {
      Some(user) => {
        command.uid(user.uid());
      }
      _ => {
        eprintln!("User {} not found", &u);
        exit(1);
      }
    }
  }
}

fn set_group(command: &mut Command, group: Option<String>) {
  if let Some(g) = group {
    match get_group_by_name(&g) {
      Some(group) => {
        command.gid(group.gid());
      }
      _ => {
        eprintln!("Group {} not found", &g);
        exit(1);
      }
    }
  }
}
