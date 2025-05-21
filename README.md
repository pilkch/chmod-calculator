[![Crates.io](https://img.shields.io/crates/v/chmod_calculator.svg)](https://crates.io/crates/chmod_calculator) [![Docs.rs](https://docs.rs/chmod_calculator/badge.svg)](https://docs.rs/chmod_calculator)

## chmod-calcuator

Basic chmod calculator. Can convert between octal 0755 style and rwxr-xr-x style and can print a table of the permissions.

### Building

Install dependencies:
```bash
sudo dnf install rust cargo
```

Build everything:
```bash
cargo build
```

### Tests

```bash
cargo test
```

### Usage

```bash
$ ./chmod-calculator 765
rwxrw-r-x
$ ./chmod-calculator 0765
rwxrw-r-x
$ ./chmod-calculator rwxrw-r-x
765
$ ./chmod-calculator --table 765
        Owner  Group Other
Read    x      x     x
Write   x      x
Execute x            x
```

### Rationale

I kept going to [chmod-calculator.com](https://chmod-calculator.com/) to convert to/from '0755' style permissions in ansible, but I always have a terminal open in VSCodium, so I thought why don't I just do it there?  And I wanted an excuse to mess around with rust.
