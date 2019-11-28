# dotenv-exec

Simple Rust wrapper around `execpv` (through [`std::os::unix::process::CommandExt`](https://doc.rust-lang.org/std/os/unix/process/trait.CommandExt.html#tymethod.exec)) and [dotenv-rs](https://github.com/dotenv-rs/dotenv).

This will execute a program populating envrionment variables from `.env` files. By default it will look up a file named `.env` in the current directory or any of its parents (you can disable this with `--no-default`) and load any env file specified with `-f / --file` in that order.

## TODO

- [ ] Publish binary
- [ ] Do a bit more testing
- [ ] CI?
- [ ] Make it work with `-f <(...)` or stdin (`-`)
- [ ] Examples
