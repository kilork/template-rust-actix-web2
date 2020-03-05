use anyhow::{anyhow, Result};
use pico_args::Arguments;
use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
    thread::spawn,
};

fn npm() -> String {
    env::var("NPM").unwrap_or_else(|_| "npm".to_string())
}

fn npm_cmd(cmd: &str) -> Result<()> {
    let npm = npm();
    let status = Command::new(npm)
        .current_dir(frontend_dir())
        .arg(cmd)
        .status()?;

    if !status.success() {
        return Err(anyhow!("'npm {}' failed", cmd));
    }

    Ok(())
}

fn npm_start() -> Result<()> {
    npm_cmd("start")
}

fn npm_install() -> Result<()> {
    npm_cmd("install")
}

fn cargo() -> String {
    env::var("CARGO").unwrap_or_else(|_| "cargo".to_string())
}

fn cargo_watch_run_backend() -> Result<()> {
    let cargo = cargo();
    let status = Command::new(cargo)
        .current_dir(project_root())
        .args(&["watch", "-x", "run --bin {{project-name}}-backend"])
        .status()?;

    if !status.success() {
        return Err(anyhow!("'cargo watch' failed"));
    }

    Ok(())
}

fn cargo_cmd(cmd: &str) -> Result<()> {
    let cargo = cargo();
    let status = Command::new(cargo)
        .current_dir(project_root())
        .arg(cmd)
        .status()?;

    if !status.success() {
        return Err(anyhow!("'cargo {}' failed", cmd));
    }

    Ok(())
}

fn cargo_build() -> Result<()> {
    cargo_cmd("build")
}

fn dir_clean<P: AsRef<Path>>(p: P) -> Result<()> {
    if p.as_ref().exists() {
        std::fs::remove_dir_all(p)?;
    }

    Ok(())
}

fn npm_clean() -> Result<()> {
    dir_clean(frontend_dir().join("node_modules"))
}

fn cargo_clean() -> Result<()> {
    cargo_cmd("clean")
}

fn cargo_install_watch() -> Result<()> {
    let cargo_cmd = cargo();
    let status = Command::new(cargo_cmd)
        .args(&["watch", "--version"])
        .status()?;

    if status.success() {
        return Ok(());
    }

    let status = Command::new(cargo()).args(&["install", "watch"]).status()?;

    if !status.success() {
        return Err(anyhow!("'cargo install watch' failed"));
    }

    Ok(())
}

pub fn project_root() -> PathBuf {
    Path::new(
        &env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned()),
    )
    .ancestors()
    .nth(1)
    .unwrap()
    .to_path_buf()
}

fn frontend_dir() -> PathBuf {
    project_root().join("frontend/web")
}

fn main() -> Result<()> {
    let mut args = Arguments::from_env();
    let subcommand = args.subcommand()?.unwrap_or_default();

    match subcommand.as_str() {
        "clean" => {
            args.finish()?;
            spawn(cargo_clean).join()?;
            npm_clean()?;
        }
        "install" => {
            args.finish()?;
            cargo_install_watch()?;
            npm_install()?;
        }
        "run" => {
            args.finish()?;

            if let Err(err) = spawn(cargo_build).join() {
                eprintln!("Cannot build, ignore for now: {:?}", err);
            }

            let npm_task = spawn(run_npm);

            let cargo_task = spawn(cargo_watch_run_backend);

            npm_task.join().expect("cannot join npm")?;
            cargo_task.join().expect("cannot join cargo")?;
        }
        "serve" => {
            args.finish()?;
            npm_run()?;
        }
        _ => {
            eprintln!(
                "\
cargo xtask
Run custom build command.

USAGE:
    cargo xtask <SUBCOMMAND>

SUBCOMMANDS:
    clean
    install
    run
    serve"
            );
        }
    }

    Ok(())
}