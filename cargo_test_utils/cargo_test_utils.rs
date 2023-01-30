use std::process::Command;
use std::process::ExitStatusError;

pub fn cmd(prog: &str, args: &[&str]) -> Command {
    let mut prog_cmd = Command::new(prog);
    prog_cmd.args(args);
    prog_cmd
}

pub fn ocamlopt_cmd(args: &[&str]) -> Command {
    cmd("ocamlopt.opt", args)
}

pub fn sh_cmd(args: &[&str]) -> Command {
    cmd("sh", args)
}

pub fn cargo_cmd(args: &[&str]) -> Command {
    cmd("cargo", args)
}

pub fn workspace_dir(ds: &[&str]) -> std::path::PathBuf {
    let mut cargo_cmd = cargo_cmd(&["locate-project", "--workspace", "--message-format=plain"]);
    let output = cargo_cmd.output().unwrap().stdout;
    let root_cargo_toml = std::path::Path::new(std::str::from_utf8(&output).unwrap().trim());
    let mut p = root_cargo_toml.parent().unwrap().to_path_buf();
    for d in ds {
        p.push(d);
    }
    p
}

pub fn run(mut cmd: Command) -> Result<(), ExitStatusError> {
    cmd.spawn().unwrap().wait().ok().unwrap().exit_ok()
}

pub fn fmt_exit_status_err(err: ExitStatusError) -> String {
    format!("error status: {err}")
}

pub fn build_flavor() -> &'static str {
    if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    }
}
