#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

/// Parses an octal string
///
/// ```
/// use chmod_calculator::parse_octal_string;
///
/// assert_eq!("765", parse_octal_string("765"));
/// assert_eq!("765", parse_octal_string("0765"));
/// ```
pub fn parse_octal_string(input: String) -> Result<String, &'static str> {
  match input.parse::<i32>() {
    Ok(parsed_number) => {
      if (parsed_number < 0) || (parsed_number > 777) {
        return Err("Invalid value");
      }

      return Ok(parsed_number.to_string());
    }
    Err(_err) => Err("Not a number"),
  }
}

/// Converts an octal chmod value to a rwx string
///
/// ```
/// use chmod_calculator::octal_to_rwx_string;
///
/// assert_eq!("--x-wxrwx", octal_to_rwx_string("123"));
/// assert_eq!("rwxrw-r-x", octal_to_rwx_string("765"));
/// ```
pub fn octal_string_to_rwx_string(input: String) -> Result<String, &'static str> {
  let mut rwx_string = String::new();

  for c in input.chars() {
    match c {
      '7' => rwx_string.push_str("rwx"),
      '6' => rwx_string.push_str("rw-"),
      '5' => rwx_string.push_str("r-x"),
      '4' => rwx_string.push_str("r--"),
      '3' => rwx_string.push_str("-wx"),
      '2' => rwx_string.push_str("-w-"),
      '1' => rwx_string.push_str("--x"),
      '0' => rwx_string.push_str("---"),
      _ => {
        return Err("Invalid character");
      }
    }
  }

  return Ok(rwx_string.to_string());
}

/// Converts a rwx string to an octal chmod value
///
/// ```
/// use chmod_calculator::rwx_string_to_octal_string;
///
/// assert_eq!("137", rwx_string_to_octal_string("--x-wxrwx"));
/// assert_eq!("765", rwx_string_to_octal_string("rwxrw-r-x"));
/// ```
pub fn rwx_string_to_octal_string(input: String) -> Result<String, &'static str> {
  let mut octal_string = String::new();

  // Split up the user, group, other parts
  let mut parts = Vec::new();
  parts.push(&input[0..3]);
  parts.push(&input[3..6]);
  parts.push(&input[6..9]);

  for p in parts {
    let p_string: String = p.to_string();
    match p_string.as_str() {
      "rwx" => octal_string.push('7'),
      "rw-" => octal_string.push('6'),
      "r-x" => octal_string.push('5'),
      "r--" => octal_string.push('4'),
      "-wx" => octal_string.push('3'),
      "-w-" => octal_string.push('2'),
      "--x" => octal_string.push('1'),
      "---" => octal_string.push('0'),
      _ => {
        println!("Err {}", p_string);
        return Err("Invalid characters");
      }
    }
  }

  return Ok(octal_string.to_string());
}



/// Prints out an rwx string as a table
///
/// Output:
///         Owner  Group Other
/// Read    x      x     x
/// Write   x      x
/// Execute x            x
///
/// ```
/// use chmod_calculator::rwx_string_to_table;
///
/// assert_eq!(765, rwx_string_to_table("--x-wxrwx"));
/// ```
pub fn rwx_string_to_table(input: String) -> Result<String, &'static str> {
  let mut x_and_spaces = String::new();

  for c in input.chars() { 
    match c {
      'w' | 'r' | 'x' => {
        x_and_spaces.push('x');
      }
      _ => {
        x_and_spaces.push(' ');
      }
    }
  }

  // There must be a better way to do this
  let output = format!(
    "        Owner  Group Other\n\
    Read    {}      {}     {}\n\
    Write   {}      {}     {}\n\
    Execute {}      {}     {}\n",
    x_and_spaces.chars().nth(0).expect("Not enough characters").to_string(), x_and_spaces.chars().nth(3).expect("Not enough characters").to_string(), x_and_spaces.chars().nth(6).expect("Not enough characters").to_string(),
    x_and_spaces.chars().nth(1).expect("Not enough characters").to_string(), x_and_spaces.chars().nth(4).expect("Not enough characters").to_string(), x_and_spaces.chars().nth(7).expect("Not enough characters").to_string(),
    x_and_spaces.chars().nth(2).expect("Not enough characters").to_string(), x_and_spaces.chars().nth(5).expect("Not enough characters").to_string(), x_and_spaces.chars().nth(8).expect("Not enough characters").to_string()
  );

  return Ok(output);
}

// 765
// rwxrw-r-x
// $ ./chmod-calculator 0765
// rwxrw-r-x
// $ ./chmod-calculator rwxrw-r-x
// 765
// $ ./chmod-calculator --table 765

pub fn convert(input: String, table: bool) -> Result<String, &'static str> {
  match parse_octal_string(input.to_string()) {
    Ok(octal_value)  => {
      // Octal string
      let rwx_string = octal_string_to_rwx_string(octal_value);
      if table {
        return rwx_string_to_table(rwx_string.unwrap());
      } else {
        return rwx_string;
      }
    }
    Err(_e) => {
      // rwx string
      let rwx_string = input;
      if table {
        return rwx_string_to_table(rwx_string.to_string());
      } else {
        return rwx_string_to_octal_string(rwx_string);
      }
    }
  };
}
