[![Build Status](https://travis-ci.org/otavio/easy-process-rs.svg?branch=master)](https://travis-ci.org/otavio/easy-process-rs)

# easy_process

Allow running external commands.

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
