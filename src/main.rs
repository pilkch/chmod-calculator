use std::env;

const EXIT_SUCCESS: i32 = 0;
const EXIT_FAILURE: i32 = 1;
const EX_USAGE: i32 = 64;

fn print_usage()
{
  eprintln!("Usage: ./chmod-calculator [OPTION] VALUE");
  eprintln!();
  eprintln!("  -v, --version   Print the version and exit");
  eprintln!("  -h, --help   Print this help and exit");
  eprintln!("  -t, --table   Print this help and exit");
  eprintln!("  -t, --table   Print this help and exit");
  eprintln!("  VALUE: The value to convert, an octal chmod value or a rwx pattern eg. 765, 0765, rwxrw-r-x");
  eprintln!();
  eprintln!("Example usage:");
  eprintln!(" $ ./chmod-calculator 765");
  eprintln!(" $ ./chmod-calculator 0765");
  eprintln!(" $ ./chmod-calculator rwxrw-r-x");
  eprintln!(" $ ./chmod-calculator --table 765");
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
  let mut output_table: bool = false;

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
      "-t" | "--table" => {
        output_table = true;
      }
      _ => {
        chmod_value = argument.to_owned();
      }
    }
  }


  // Do the conversion
  let result = chmod_calculator::convert(chmod_value, output_table);
  match result {
    Ok(output) => println!("{}", output),
    Err(e) => {
      println!("{}", e);
      std::process::exit(EXIT_FAILURE);
    }
  }
  std::process::exit(EXIT_SUCCESS);
}

