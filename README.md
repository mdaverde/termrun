<div align="center">
	<h1>termrun</h1>
	<p>
        Send & run commands on other open Unix terminals
	</p>
	<br>
</div>

## Usage

```shell
$ termrun
```

__Note__: Requires `sudo` each time unless root is set as owner. See Privileges section below

### Quick example

Terminal 1:
```shell
$ tty
/dev/pts/2
```

Terminal 2:
```shell
$ termrun --newline --pty /dev/pts/2 echo hello world 
```

(`--newline` appends `\n` to the sent command to actually run)

Terminal 1:
```shell
$ tty
/dev/pts/2
$ echo hello world 
hello world 
```

## Install

### Cargo

If you're using a recent version of Cargo, you can see the `cargo install` command:

```shell
$ cargo install termrun 
```

### Build from source

After git cloning this repo, you can install as a cargo crate through

```shell
$ cargo install --path path/to/repo
```

This should make `termrun` available everywhere assuming your cargo crates are in `$PATH`

### Privileges / Post-installation

`termrun` uses `ioctl(2)` under the hood through the [TIOCSTI](https://man7.org/linux/man-pages/man4/tty_ioctl.4.html) cmd flag. To do this successfully, the process needs root user privileges to run.

In practice, this means having to run `termrun` with `sudo`. By default when you install global crates, sudo doesn't know about them:

```shell
$ termrun
Error: EPERM: Operation not permitted

$ sudo termrun
[sudo] password for user: 
sudo: termrun: command not found
```

The solutions here are to:

- (Easiest) Symlink `termrun` into a `sudo`-friendly path: `sudo ln -s ~/.cargo/bin/termrun /usr/local/bin/`
- Always specify the complete crate path so `sudo` can find `termrun`: `sudo ~/.cargo/bin/termrun`
- Specify `sudo` to use your `$PATH`: `sudo env "PATH=$PATH" termrun`

## License

MIT - Maintained by [Milan](https://mdaverde.com)

