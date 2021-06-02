use std::{env, ffi, fs, process};

use libc;
use nix;
use anyhow::{anyhow, Result};

use ops::Op;

mod ops;

static VERSION: &'static str = env!("CARGO_PKG_VERSION");

static USAGE: &str = "(TODO) Incorrect usage"; // TODO

fn parse_args(mut args: Vec<ffi::OsString>) -> Result<Op> {
    println!("args: {:?}", args);

    if args.len() < 2 {
        return Ok(Op::Interactive);
    } else if args.len() == 2 {
        let opt = &args[1];
        return if opt == "--help" || opt == "-h" {
            Ok(Op::Help)
        } else if opt == "--version" || opt == "-v" {
            Ok(Op::Version)
        } else if opt == "--interactive" || opt == "-i" {
            Ok(Op::Interactive)
        } else {
            Err(anyhow::anyhow!("Did not understand passed in arg"))
        }
    }

    let tty = args.remove(1);
    let cmd = args.drain(1..).collect();

    Ok(Op::Single(tty, cmd, false))
}

fn main() {
    match parse_args(env::args_os().collect()) {
        Err(err) => {
            println!("Error: {:?}", err);
            println!("{}", USAGE);
            process::exit(1);
        }
        Ok(Op::Help) => {
            println!("{}", USAGE);
        }
        Ok(Op::Version) => {
            println!("{}", VERSION);
        }
        Ok(Op::Interactive) => if let Err(err) = ops::interactive() {},
        Ok(Op::All(cmd, newline)) => if let Err(err) = ops::all(cmd, newline) {},
        Ok(Op::Single(pty_path, cmd, newline)) => {
            if let Err(err) = ops::single(pty_path.into(), cmd, newline) {
                println!("Error: {:?}", err);
                process::exit(1);
            }
        }
    }
}
