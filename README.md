# dotenv-exec

![Crates.io](https://img.shields.io/crates/v/dotenv-exec)
![GitHub Workflow Status](https://img.shields.io/github/workflow/status/lirsacc/dotenv-exec/ci)

Simple Rust wrapper around `execpv` (through [`std::os::unix::process::CommandExt`](https://doc.rust-lang.org/std/os/unix/process/trait.CommandExt.html#tymethod.exec)) and [dotenv-rs](https://github.com/dotenv-rs/dotenv) for unix systems.

This will execute a program populating environment variables from `.env` files. By default it will look up a file named `.env` in the current directory or any of its parents (you can disable this with `--no-default`) and load any env file specified with `-f / --file` in that order.

All formatting, substitution and ordering rules are the same as `dotenv-rs`.

## Installation

- Install through cargo: `cargo install dotenv-exec`
- Grab binaries from the [Github releases](https://github.com/lirsacc/dotenv-exec/releases) page.

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

## Notes

- This is a first version and there might be some changes based on how my usage evolves. Specifically I am not sure `--no-default` and `--ignore-missing` are the right defaults and I see a risk that the no override / reverse priority order behaviour could be counter intuitive.
- As reading from strings rather than files [doesn't seem supported by dotenv-rs for now](https://github.com/dotenv-rs/dotenv/issues/15) this doesn't implement the `-f -` convention. IO redirection should work as an alternative when using dynamically generated env files.

