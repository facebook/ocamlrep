use std::process::Command;
use std::process::ExitStatusError;

#[allow(dead_code)]
fn cmd(prog: &str, args: &[&str], dir: Option<&std::path::Path>) -> Command {
    let mut prog_cmd = Command::new(prog);
    if let Some(path) = dir {
        prog_cmd.current_dir(path);
    }
    prog_cmd.args(args);
    prog_cmd
}

#[allow(dead_code)]
fn ocamlopt_cmd(args: &[&str], dir: Option<&std::path::Path>) -> Command {
    cmd("ocamlopt.opt", args, dir)
}

#[allow(dead_code)]
fn sh_cmd(args: &[&str], dir: Option<&std::path::Path>) -> Command {
    cmd("sh", args, dir)
}

#[allow(dead_code)]
fn cargo_cmd(args: &[&str]) -> Command {
    cmd("cargo", args, None)
}

#[allow(dead_code)]
fn workspace_dir(ds: &[&str]) -> std::path::PathBuf {
    let mut cargo_cmd = cargo_cmd(&["locate-project", "--workspace", "--message-format=plain"]);
    let output = cargo_cmd.output().unwrap().stdout;
    let root_cargo_toml = std::path::Path::new(std::str::from_utf8(&output).unwrap().trim());
    let mut p = root_cargo_toml.parent().unwrap().to_path_buf();
    for d in ds {
        p.push(d);
    }
    p
}

#[allow(dead_code)]
fn run(mut cmd: Command) -> Result<(), ExitStatusError> {
    cmd.spawn().unwrap().wait().ok().unwrap().exit_ok()
}

#[allow(dead_code)]
fn fmt_exit_status_err(err: ExitStatusError) -> String {
    format!("error status: {err}")
}

#[allow(dead_code)]
fn build_flavor() -> &'static str {
    if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    }
}
