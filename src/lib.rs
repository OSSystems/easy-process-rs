#![deny(
    warnings, missing_debug_implementations, missing_copy_implementations, trivial_casts,
    trivial_numeric_casts, unsafe_code, unstable_features, unused_import_braces,
    unused_qualifications, missing_docs
)]

// Copyright (C) 2018 O.S. Systems Sofware LTDA
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//

//! Allow running external commands and properly handle its success
//! and failures.
//!
//! This creates provides a `run` function that does inline parsing of
//! literal command line strings (handling escape codes and splitting
//! at whitespace) and checks the ExitStatus of the command. If it
//! didn't succeed they will return a Err(...) instead of a Ok(...).
//!
//! Note that the provided functions do return their own `Output`
//! struct instead of [`std::process::Output`].
//!
//! # Example
//! ```
//! # fn run() -> Result<(), easy_process::Error> {
//! use easy_process;
//!
//! // stdout
//! let output = easy_process::run(r#"sh -c 'echo "1 2 3 4"'"#)?;
//! assert_eq!(&output.stdout, "1 2 3 4\n");
//!
//! // stderr
//! let output = easy_process::run(r#"sh -c 'echo "1 2 3 4" >&2'"#)?;
//! assert_eq!(&output.stderr, "1 2 3 4\n");
//! # Ok(())
//! # }
//! # run();
//!```
//!
//! [`std::process::Output`]: https://doc.rust-lang.org/std/process/struct.Output.html

extern crate checked_command;
extern crate cmdline_words_parser;

use cmdline_words_parser::StrExt;
use std::error;
use std::fmt;
use std::io;
use std::process::ExitStatus;

#[derive(Debug, Default)]
/// Holds the output for a giving easy_process::run
pub struct Output {
    /// The stdout output of the process
    pub stdout: String,
    /// The stderr output of the process
    pub stderr: String,
}

#[derive(Debug)]
/// Error variant for `easy_process::run`.
pub enum Error {
    /// I/O error
    Io(io::Error),
    /// Process error. It holds two parts: first argument is the exit
    /// code and the second is the output (stdout and stderr).
    Failure(ExitStatus, Output),
}

impl From<checked_command::Error> for Error {
    fn from(error: checked_command::Error) -> Self {
        match error {
            checked_command::Error::Io(e) => Error::Io(e),
            checked_command::Error::Failure(ex, err) => Error::Failure(
                ex,
                match err {
                    Some(e) => Output {
                        stdout: String::from_utf8_lossy(&e.stdout).to_string(),
                        stderr: String::from_utf8_lossy(&e.stderr).to_string(),
                    },
                    None => Output::default(),
                },
            ),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "Process error"
    }
    fn cause(&self) -> Option<&error::Error> {
        Some(self)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "unexpected I/O Error: {}", e),
            Error::Failure(ex, output) => write!(
                f,
                "status: {:?} stdout: {:?} stderr: {:?}",
                ex.code(),
                output.stdout,
                output.stderr
            ),
        }
    }
}

/// Runs the given command
///
/// # Arguments
///
/// `cmd` - A string slice containing the command to be run.
///
/// # Errors
///
/// if the exit status is not successful or a io::Error was returned.
pub fn run(cmd: &str) -> Result<Output, Error> {
    let mut cmd = cmd.to_string();
    let mut cmd = cmd.parse_cmdline_words();

    let mut p = checked_command::CheckedCommand::new(cmd.next().unwrap());
    for arg in cmd {
        p.arg(arg);
    }

    let o = p.output()?;
    Ok(Output {
        stdout: String::from_utf8_lossy(&o.stdout).to_string(),
        stderr: String::from_utf8_lossy(&o.stderr).to_string(),
    })
}

#[test]
fn failing_command() {
    // failing command with exit status 1
    match run(r#"sh -c 'echo "error" >&2; exit 1'"#) {
        Ok(_) => panic!("call should have failed"),
        Err(Error::Io(io_err)) => panic!("unexpected I/O Error: {:?}", io_err),
        Err(Error::Failure(ex, output)) => {
            assert_eq!(ex.code().unwrap(), 1);
            assert_eq!(&output.stderr, "error\n");
        }
    }
}
