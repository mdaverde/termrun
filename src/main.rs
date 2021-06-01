use std::convert::TryInto;
use std::os::unix::prelude::*;
use std::{env, error, ffi, fs, process};

use libc;
use nix;

// use nix::{ioctl}

// enum Op {
//     Run()
// }
//
// #[derive(Debug, Display)]
// enum CliError {
//
// }
//
// impl error::Error for CliError {}
//
// fn parse_args(args: env::ArgsOs) -> Result<Op, CliError> {
//
// }

static USAGE: &str = "(TODO) Incorrect usage"; // TODO

fn tty_write(fd: RawFd, strings: Vec<ffi::OsString>, newline: bool) {
    // TODO: return errors
    nix::ioctl_write_ptr_bad!(tty_write, libc::TIOCSTI, libc::c_char);

    for string in strings {
        let string_len = string.len();
        let string_in_c = ffi::CString::new(string.into_vec()).unwrap();
        let string_ptr = string_in_c.as_ptr();

        for i in 0..string_len {
            let ii: isize = i.try_into().unwrap(); // TODO
            unsafe { tty_write(fd, string_ptr.offset(ii) as *const _) }
                .expect("Write not successful");
        }

        let nl = ffi::CString::new(" ").unwrap();
        unsafe { tty_write(fd, nl.as_ptr() as *const _) }.expect("Write not successful");
    }

    if newline {
        let nl = ffi::CString::new("\n").unwrap();
        unsafe { tty_write(fd, nl.as_ptr() as *const _) }.expect("Write not successful");
    }
}

fn main() {
    let mut args = env::args_os();
    if args.len() < 3 {
        println!("{}", USAGE);
        process::exit(1);
    }

    args.next().unwrap(); // cmd name
    let tty = args.next().expect("Did not understand specified terminal");
    let mut cmd = args.collect::<Vec<ffi::OsString>>();

    let tty_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(tty)
        .expect("Did not understand specified tty");
    let tty_fd: RawFd = tty_file.as_raw_fd();

    tty_write(tty_fd, cmd, true);

    println!("Raw fd of tty is: {}", tty_fd);
}
