use std::collections::VecDeque;
use std::ffi;
use std::os::unix::prelude::*;

use anyhow::anyhow;

use crate::ops::Op;

// This is gross but works for now; deserves a refactoring
pub fn parse_args(mut args: Vec<ffi::OsString>) -> anyhow::Result<Op> {
    if args.len() < 2 {
        return Ok(Op::Help);
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

    let mut arg_pop = arg_deque.pop_front();
    while arg_pop.is_some() {
        let arg = arg_pop.unwrap();

        if collect_pty_arg {
            if arg.as_bytes().starts_with(b"-") {
                return Err(anyhow!("Did not understand specified --pty"));
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vec_into {
        ($($x:expr),+ $(,)?) => (
            vec![$($x.into()),+]
        );
    }

    #[test]
    fn test_parse_args() {
        assert_eq!(
            parse_args(vec_into![
                "termrun",
                "--pty",
                "/dev/pts/2",
                "echo",
                "hello",
                "world"
            ])
            .unwrap(),
            Op::Single(
                "/dev/pts/2".into(),
                vec_into!["echo", "hello", "world"],
                false
            )
        );

        assert_eq!(
            parse_args(vec_into!["termrun", "-p", "/dev/pts/2", "ls"]).unwrap(),
            Op::Single("/dev/pts/2".into(), vec_into!["ls"], false)
        );

        assert_eq!(
            parse_args(vec_into!["termrun", "--all", "ls",]).unwrap(),
            Op::All(vec_into!["ls"], false)
        );

        assert_eq!(
            parse_args(vec_into!["termrun", "--newline", "--all", "ls"]).unwrap(),
            Op::All(vec_into!["ls"], true)
        );

        assert_eq!(
            parse_args(vec_into!["termrun", "-n", "-a", "ls"]).unwrap(),
            Op::All(vec_into!["ls"], true)
        );

        assert_eq!(
            parse_args(vec_into!["termrun", "--version"]).unwrap(),
            Op::Version
        );

        assert_eq!(parse_args(vec_into!["termrun", "-v"]).unwrap(), Op::Version);

        assert_eq!(parse_args(vec_into!["termrun"]).unwrap(), Op::Help);

        assert_eq!(
            parse_args(vec_into!["termrun", "--help"]).unwrap(),
            Op::Help
        );

        assert_eq!(parse_args(vec_into!["termrun", "-h"]).unwrap(), Op::Help);

        assert!(parse_args(vec_into!["termrun", "ls"]).is_err());
        assert!(parse_args(vec_into!["termrun", "ls", "--newline"]).is_err());
        assert!(parse_args(vec_into!["termrun", "--pty"]).is_err());
        assert!(parse_args(vec_into!["termrun", "--pty", "ls"]).is_err());
        assert!(parse_args(vec_into!["termrun", "--all"]).is_err());
        assert!(parse_args(vec_into!["termrun", "-p", "-a"]).is_err());
        assert!(parse_args(vec_into!["termrun", "-p", "-a", "ls"]).is_err());
        assert!(parse_args(vec_into!["termrun", "-n", "-a"]).is_err());
        assert!(parse_args(vec_into!["termrun", "--new", "-a"]).is_err());
        assert!(parse_args(vec_into!["termrun", "-n", "-p"]).is_err());
        assert!(parse_args(vec_into!["termrun", "--newline", "-p"]).is_err());
    }
}
