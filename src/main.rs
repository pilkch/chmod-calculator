use std::env;

const EXIT_SUCCESS: i32 = 0;
//const EXIT_FAILURE: i32 = 1;
const EX_USAGE: i32 = 64;

fn print_usage()
{
  eprintln!("Usage: ./chmod-calculator [OPTION]");
  eprintln!();
  eprintln!("  -v, --version   Print the version and exit");
  eprintln!("  -h, --help   Print this help and exit");
}

fn print_version()
{
  const VERSION: &str = env!("CARGO_PKG_VERSION");

  println!("chmod-calculator version {}", VERSION);
}

fn main() {
  let args: Vec<_> = env::args().skip(1).collect();
  if args.len() < 1 {
    // Incorrect number of arguments, print the usage and exit
    print_usage();
    std::process::exit(EX_USAGE);
  }

  let mut chmod_value: String = "".to_string();
  let mut table: bool = false;

  for argument in args {
    match argument.as_str() {
      "-h" | "--help" => {
        print_usage();
        std::process::exit(EXIT_SUCCESS);
      }
      "-v" | "--version" => {
        print_version();
        std::process::exit(EXIT_SUCCESS);
      }
      "--table" => {
        table = true;
        println!("table found {}", table);
      }
      _ => {
        chmod_value = argument.to_owned();
        println!("chmod value found {}", chmod_value);
      }
    }
  }

  println!("chmod: {}, table: {}", chmod_value, table);
}

