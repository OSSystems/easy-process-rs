[![Build Status](https://travis-ci.org/otavio/process.svg?branch=master)](https://travis-ci.org/otavio/process)

# process

Allow running external commands.

### Examples
```rust
use process;

// stdout
let output = process::run(r#"sh -c 'echo "1 2 3 4"'"#)?;
assert_eq!(&output.stdout, "1 2 3 4\n");

// stderr
let output = process::run(r#"sh -c 'echo "1 2 3 4" >&2'"#)?;
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
