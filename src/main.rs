use std::collections::VecDeque;
use std::env::args_os;
use std::os::unix::prelude::*;
use std::{env, ffi, process};

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
            Err(anyhow!("Did not understand passed in args"))
        };
    }

    args.remove(0); // cmd name

    let mut newline = false;
    let mut all_ptys = false;
    let mut single_pty = false;
    let mut collect_pty_arg = false;
    let mut single_pty_path: Option<ffi::OsString> = None;

    let mut arg_deque = VecDeque::from(args);
    println!("args: {:?}", arg_deque);

    let mut arg_pop = arg_deque.pop_front();
    while arg_pop.is_some() {
        let arg = arg_pop.unwrap();

        if collect_pty_arg {
            single_pty_path = Some(arg);
            collect_pty_arg = false;
        } else {
            let bytes = arg.as_bytes();

            if !bytes.starts_with(b"-") {
                arg_deque.push_front(arg);
                break;
            } else if arg == "--newline" || arg == "-n" {
                newline = true;
            } else if arg == "--all" || arg == "-a" {
                all_ptys = true;
            } else if arg == "--pty" || arg == "-p" {
                single_pty = true;
                collect_pty_arg = true;
            } else {
                break;
            }
        }

        arg_pop = arg_deque.pop_front();
    }

    println!(
        "About to run: {:?} {:?} {}",
        single_pty_path, arg_deque, newline
    );

    if single_pty {
        if all_ptys {
            return Err(anyhow!("Specified both --pty and --all"));
        } else if single_pty_path.is_none() {
            return Err(anyhow!("Specified --pty but with no terminal path"));
        }
    }
    if arg_deque.len() < 1 {
        return Err(anyhow!("Did not specify cmd to send to terminals"));
    }

    if all_ptys {
        return Ok(Op::All(arg_deque.into(), newline));
    } else if single_pty {
        return Ok(Op::Single(
            single_pty_path.unwrap(),
            arg_deque.into(),
            newline,
        ));
    }

    return Err(anyhow!("Had trouble parsing arguments given"));
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
        Ok(Op::Interactive) => if let Err(_err) = ops::interactive() {},
        Ok(Op::All(cmd, newline)) => if let Err(_err) = ops::all(cmd, newline) {},
        Ok(Op::Single(pty_path, cmd, newline)) => {
            if let Err(err) = ops::single(pty_path.into(), cmd, newline) {
                println!("Error: {:?}", err);
                process::exit(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_parse_args() {}
}
