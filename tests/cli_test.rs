//! Command line interface integration tests
//!
//! Here we're using assert_cmd to test the command line interface of our main binary
//! https://github.com/assert-rs/assert_cmd
//! https://rust-lang-nursery.github.io/cli-wg/tutorial/testing.html

use std::process::Command;
use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};

#[test]
fn prints_the_output_of_octal() {
  let mut command = Command::cargo_bin("chmod-calculator").unwrap();
  command.args(&["765"]).assert().success().stdout("rwxrw-r-x\n");
}

#[test]
fn prints_the_output_of_octal_with_zero() {
  let mut command = Command::cargo_bin("chmod-calculator").unwrap();
  command.args(&["0765"]).assert().success().stdout("rwxrw-r-x\n");
}

#[test]
fn prints_the_output_of_rwx() {
  let mut command = Command::cargo_bin("chmod-calculator").unwrap();
  command.args(&["--x-wxrwx"]).assert().success().stdout("137\n");
}

#[test]
fn prints_the_output_of_octal_with_zero_table() {
  let mut command = Command::cargo_bin("chmod-calculator").unwrap();
  command.args(&["--table", "0451"]).assert().success().stdout("        Owner  Group Other\n\
Read    x      x      \n\
Write                 \n\
Execute        x     x\n");
}

#[test]
fn prints_the_output_of_rwx_table() {
  let mut command = Command::cargo_bin("chmod-calculator").unwrap();
  command.args(&["--table", "--x-wxrwx"]).assert().success().stdout("        Owner  Group Other\n\
Read                 x\n\
Write          x     x\n\
Execute x      x     x\n");
}


// Failure tests
#[test]
fn prints_an_error_when_argument_is_not_a_number() {
  let mut command = Command::cargo_bin("chmod-calculator").unwrap();
  command
    .env_remove("RUST_BACKTRACE") // Ensure that the whole backtrace doesn't get printed
    .args(&["this_is_an_invalid_argument"])
    .assert()
      .failure()
      .code(1)
      .stderr("Invalid value\n");
}
