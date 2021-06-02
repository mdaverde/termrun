use std::convert::TryInto;
use std::os::unix::prelude::*;
use std::{env, ffi, fs, path, process};

use anyhow::{anyhow, Context, Result};

pub enum Op {
    Single(ffi::OsString, Vec<ffi::OsString>, bool),
    All(Vec<ffi::OsString>, bool),
    Interactive,
    Help,
    Version,
}

fn pty_input_ffi(cmd: Vec<ffi::OsString>, newline: bool) -> Result<ffi::CString> {
    let mut input: Vec<u8> = vec![];

    let len = cmd.len();

    for (ind, arg) in cmd.into_iter().enumerate() {
        let mut arg_vec = arg.into_vec();
        input.append(&mut arg_vec);
        if ind < (len - 1) {
            input.push(b' ');
        }
    }

    if newline {
        input.push(b'\n');
    }

    return ffi::CString::new(input).context("Did not understand cmds given");
}

fn pty_write(fd: RawFd, input: &ffi::CString) -> Result<()> {
    // Generates ioctl fn
    nix::ioctl_write_ptr_bad!(inner_pty_write, libc::TIOCSTI, libc::c_char);

    let len = input.as_bytes().len();
    let ptr = input.as_ptr();

    for u_index in 0..len {
        let i_index: isize = u_index
            .try_into()
            .context("Command passed in was too long")?;
        unsafe { inner_pty_write(fd, ptr.offset(i_index) as *const _) }?;
    }

    Ok(())
}

struct Pty(path::PathBuf);

fn get_all_ptys() -> Result<Vec<Pty>> {
    let dir = path::PathBuf::from("/dev/pts");
    if !dir.exists() {
        return Err(anyhow!("/dev/pts does not exist"));
    }
    let ptys = fs::read_dir(dir)
        .context("Could not open /dev/pts")?
        .map(|res| {
            res.context("Could not read all files in /dev/pts successfully")
                .map(|entry| Pty(entry.path()))
        })
        .collect::<Result<Vec<Pty>>>()
        .context("Could not open /dev/pts")?;

    Ok(ptys)
}

pub fn interactive() -> Result<()> {
    Ok(())
}

pub fn all(cmd: Vec<ffi::OsString>, newline: bool) -> Result<()> {
    let ptys = get_all_ptys()?;
    let pty_input = pty_input_ffi(cmd, newline)?;

    for Pty(pty_path) in ptys {
        let pty_file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(pty_path)
            .context(format!("Could not open terminal path given"))?; // TODO
        pty_write(pty_file.as_raw_fd(), &pty_input)?;
    }

    Ok(())
}

pub fn single(pty_path: path::PathBuf, cmd: Vec<ffi::OsString>, newline: bool) -> Result<()> {
    let pty_input = pty_input_ffi(cmd, newline)?;
    let pty_file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(pty_path)
        .context(format!("Could not open terminal path given"))?; // TODO
    pty_write(pty_file.as_raw_fd(), &pty_input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pty_input() {
        let c_str = pty_input_ffi(
            vec![
                ffi::OsString::from("echo"),
                ffi::OsString::from("hello"),
                ffi::OsString::from("world"),
            ],
            false,
        )
        .unwrap();

        println!("cstr: {:?}", c_str);

        assert_eq!(c_str, ffi::CString::new("echo hello world").unwrap());
    }
}
