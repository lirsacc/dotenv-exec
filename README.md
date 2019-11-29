# dotenv-exec

Simple Rust wrapper around `execpv` (through [`std::os::unix::process::CommandExt`](https://doc.rust-lang.org/std/os/unix/process/trait.CommandExt.html#tymethod.exec)) and [dotenv-rs](https://github.com/dotenv-rs/dotenv) for unix systems.

This will execute a program populating environment variables from `.env` files. By default it will look up a file named `.env` in the current directory or any of its parents (you can disable this with `--no-default`) and load any env file specified with `-f / --file` in that order.

All formatting, substitution and ordering rules are the same as `dotenv-rs`.

## Examples

```bash
$ cat <<EOT > .env
VAR_1=1
VAR_2=2
EOT

$ cat <<EOT > .env-2
VAR_1=0
VAR_3=3
EOT

# Load .env by default
$ dotenv-exec -- env | grep VAR_
VAR_1=1
VAR_2=2

# Disable this behaviour with --no-default
$ dotenv-exec --no-default -- env | grep VAR_

# dotenv-rs does not override already set values (see VAR_1), so the order in
# which files are specified is important.
$ dotenv-exec -f .env-2 -- env | grep VAR_
VAR_1=1
VAR_2=2
VAR_3=3

$ dotenv-exec --no-default -f .env-2 -f .env -- env | grep VAR_
VAR_1=0
VAR_3=3
VAR_2=2

# If you generate env file on the fly (e.g. decrypting them), you should use
# IO redirection
dotenv-exec --no-default -f <(cat .env-2) -- env | grep VAR_
VAR_1=0
VAR_3=3
```

## TODO

- [ ] Publish binary
- [ ] Do a bit more testing
- [x] CI -> [Github Actions](https://github.com/lirsacc/dotenv-exec/actions)
- [x] Make sure it works with `-f <(...)`
- [x] ~~Make sure it works with stdin (`-`)~~ (reading from strings rather than files [doesn't seem supported for now](https://github.com/dotenv-rs/dotenv/issues/15))
- [x] Examples
- [ ] Consider whether `--no-default` is the correct approach
