<div align="center">
	<h1>termrun</h1>
	<p>
        Run a command in another terminal (or all of them)
	</p>
	<br>
</div>

## Usage

```shell
$ termrun
```

Requires `sudo` each time unless root is set as owner

### Example

TODO: insert gif/ascii video

```shell
$ termrun /dev/pts/2 ls
```

### Flags

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


## License

MIT - Maintained by [Milan](https://mdaverde.com)

