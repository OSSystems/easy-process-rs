[![Coverage Status](https://coveralls.io/repos/github/OSSystems/easy-process-rs/badge.svg?branch=master)](https://coveralls.io/github/OSSystems/easy-process-rs?branch=master)
[![Documentation](https://docs.rs/easy_process/badge.svg)](https://docs.rs/easy_process)

# easy_process

Allow running external commands and properly handle its success
and failures.

| Platform | Build Status |
| -------- | ------------ |
| Linux | [![build status](https://github.com/OSSystems/easy-process-rs/workflows/CI%20(Linux)/badge.svg)](https://github.com/OSSystems/easy-process-rs/actions) |
| macOS | [![build status](https://github.com/OSSystems/easy-process-rs/workflows/CI%20(macOS)/badge.svg)](https://github.com/OSSystems/easy-process-rs/actions) |
| Windows | [![build status](https://github.com/OSSystems/easy-process-rs/workflows/CI%20(Windows)/badge.svg)](https://github.com/OSSystems/easy-process-rs/actions) |

---

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

Commands on windows are also supported in the same way:

```rust
let output = easy_process::run(r#"powershell /C 'echo "1 2 3 4"'"#)?;
assert_eq!(&output.stdout, "1 2 3 4\r\n");
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
