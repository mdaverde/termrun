use std::{env, process};

use ops::Op;

mod ops;
mod parse;

static VERSION: &str = env!("CARGO_PKG_VERSION");

// TODO
static USAGE: &str = "termrun - send a command to other open Unix terminals";

fn main() {
    match parse::parse_args(env::args_os().collect()) {
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
        Ok(Op::All(cmd, newline)) => {
            if let Err(err) = ops::all(cmd, newline) {
                println!("Error: {:?}", err);
                process::exit(1);
            }
        }
        Ok(Op::Single(pty_path, cmd, newline)) => {
            if let Err(err) = ops::single(pty_path.into(), cmd, newline) {
                println!("Error: {:?}", err);
                process::exit(1);
            }
        }
    }
}
