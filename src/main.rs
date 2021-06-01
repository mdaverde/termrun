use std::convert::TryInto;
use std::os::unix::prelude::*;
use std::{env, ffi, fs, process};

use libc;
use nix;

use error::CliError;

mod error;

enum Op {
    SingleTerm(ffi::OsString, Vec<ffi::OsString>, bool),
}

fn parse_args(mut args: env::ArgsOs) -> Result<Op, CliError> {
    if args.len() < 3 {
        println!("{}", USAGE);
        process::exit(1);
    }

    args.next().unwrap(); // cmd name

    let tty = args.next().expect("Did not understand specified terminal");

    let cmd = args.collect::<Vec<ffi::OsString>>();

    Ok(Op::SingleTerm(tty, cmd, false))
}

static USAGE: &str = "(TODO) Incorrect usage"; // TODO

fn tty_write(fd: RawFd, strings: Vec<ffi::OsString>, newline: bool) -> Result<(), CliError> {
    nix::ioctl_write_ptr_bad!(inner_tty_write, libc::TIOCSTI, libc::c_char);

    for string in strings {
        let string_len = string.len();
        let string_in_c = ffi::CString::new(string.into_vec())?;
        let string_ptr = string_in_c.as_ptr();

        for i in 0..string_len {
            let ii: isize = i.try_into().or(Err(CliError::TooLongCmd))?; // TODO
            unsafe { inner_tty_write(fd, string_ptr.offset(ii) as *const _) }?;
        }

        let nl = ffi::CString::new(" ")?;
        unsafe { inner_tty_write(fd, nl.as_ptr() as *const _) }?;
    }

    if newline {
        let nl = ffi::CString::new("\n")?;
        unsafe { inner_tty_write(fd, nl.as_ptr() as *const _) }?;
    }

    Ok(())
}

fn main() {
    match parse_args(env::args_os()) {
        Err(err) => {
            println!("Error: {:?}", err);
            println!("{}", USAGE);
            process::exit(1);
        }

        Ok(Op::SingleTerm(tty_path, cmd, newline)) => {
            let tty_file = fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open(tty_path)
                .expect("Did not understand specified tty");

            let tty_fd: RawFd = tty_file.as_raw_fd();

            if let Err(err) = tty_write(tty_fd, cmd, newline) {
                println!("Error: {:?}", err);
                process::exit(1);
            }
        }
    }
}
