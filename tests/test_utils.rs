use std::process::Command;

pub fn run_userkit_command(subcommands: Vec<&str>) -> Command {
  let mut cmd = Command::new("./target/debug/userkit");

  for subcmd in subcommands {
    cmd.arg(subcmd);
  }

  cmd
}

pub fn sudo_run_userkit_command(subcommands: Vec<&str>) -> Command {
  let mut cmd = Command::new("sudo");
  cmd.arg("./target/debug/userkit");

  for subcmd in subcommands {
    cmd.arg(subcmd);
  }

  cmd
}
