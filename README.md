[![Build Status](https://travis-ci.org/otavio/easy-process-rs.svg?branch=master)](https://travis-ci.org/otavio/easy-process-rs) [![Documentation](https://docs.rs/easy_process/badge.svg)](https://docs.rs/easy_process)

# easy_process

Allow running external commands and properly handle its success
and failures.

This creates provides a `run` function that does inline parsing of
literal command line strings (handling escape codes and splitting
at whitespace) and checks the `ExitStatus` of the command. If it
didn't succeed they will return a `Err(...)` instead of a
`Ok(...)`.

Note that the provided functions do return their own `Output`
struct instead of [`std::process::Output`].

## Example
```rust
use easy_process;

// stdout
let output = easy_process::run(r#"sh -c 'echo "1 2 3 4"'"#)?;
assert_eq!(&output.stdout, "1 2 3 4\n");

// stderr
let output = easy_process::run(r#"sh -c 'echo "1 2 3 4" >&2'"#)?;
assert_eq!(&output.stderr, "1 2 3 4\n");
```

[`std::process::Output`]: https://doc.rust-lang.org/std/process/struct.Output.html

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
