use std::num::ParseIntError;

type Number = i64;
type CalculatorResult = Result<Number, CalculatorError>;

#[derive(Debug)]
enum CalculatorError {
  ParseDelimiterError,
  ParseIntError(ParseIntError),
}
impl From<ParseIntError> for CalculatorError {
  fn from(err: ParseIntError) -> Self {
    CalculatorError::ParseIntError(err)
  }
}

/// Free-function that can take two numbers, separated by commas, and will return their sum
fn add(numbers: &str) -> CalculatorResult {
  if numbers.is_empty() {
    Ok(0)
  } else {
    add_non_empty(numbers)
  }
}

fn add_non_empty(numbers: &str) -> CalculatorResult {
  // Need 2 variables becuase 2 possible closures will have 2 different types
  // This solution was picked instead of Boxing the closures as it doesn't involve a dynamic allocation
  // https://stackoverflow.com/questions/27890435/assigning-an-unboxed-closure-from-an-if-statement
//  let default_is_delimiter;
//  let custom_is_delimiter;
//  let is_delimiter = if numbers.starts_with("//") {
//    // Set delimiter if configured by the first line
//    let lines = numbers.splitn(2, '\n').collect::<Vec<&str>>();
//    let delimiter_line = lines.first().ok_or(CalculatorError::ParseDelimiterError)?;
//    let delimiter_char = delimiter_line.trim_start_matches('/').chars().next().ok_or(CalculatorError::ParseDelimiterError)?;
//    custom_is_delimiter = |&c| c == delimiter_char;
//    // Note: If not reference, the following error occurs
//    // error[E0620]: cast to unsized type: `[closure@string-calculator/src/lib.rs:37:27: 37:45 delimiter:_]` as `dyn std::ops::Fn() -> _`
//    &custom_is_delimiter as &dyn Fn(char) -> _
//  } else {
//    default_is_delimiter = |&c| c == ',' || c == '\n';
//    &default_is_delimiter as &dyn Fn(char) -> _
//  };
//
//  // TODO: Check if collect() call in the middle makes this code less performant
//  Ok(numbers.split(is_delimiter).map(|s| s.parse::<Number>()).collect::<Result<Vec<Number>, ParseIntError>>()?.iter().fold(0, |acc, num| acc + num))
  if numbers.starts_with("//") {
    let lines = numbers.splitn(2, '\n').collect::<Vec<&str>>();
    let delimiter_line = lines.first().ok_or(CalculatorError::ParseDelimiterError)?;
    let numbers_line = lines.last().ok_or(CalculatorError::ParseDelimiterError)?;
    let delimiter_char = delimiter_line.trim_start_matches('/').chars().next().ok_or(CalculatorError::ParseDelimiterError)?;
    // TODO: Check if collect() call in the middle makes this code less performant
    Ok(numbers_line.split(|c| c == delimiter_char).map(|s| s.parse::<Number>()).collect::<Result<Vec<Number>, ParseIntError>>()?.iter().fold(0, |acc, num| acc + num))
  } else {
    Ok(numbers.split(|c| c == ',' || c == '\n').map(|s| s.parse::<Number>()).collect::<Result<Vec<Number>, ParseIntError>>()?.iter().fold(0, |acc, num| acc + num))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn add_empty_string() -> Result<(), CalculatorError> {
    assert_eq!(add("")?, 0);
    Ok(())
  }

  #[test]
  fn add_one_number_string() -> Result<(), CalculatorError> {
    assert_eq!(add("1")?, 1);
    Ok(())
  }

  #[test]
  fn add_one_invalid_string() -> Result<(), CalculatorError> {
    assert!(matches!(add("a"), Err(CalculatorError::ParseIntError(_))));
    Ok(())
  }

  #[test]
  fn add_two_numbers_string_1() -> Result<(), CalculatorError> {
    assert_eq!(add("1,2")?, 3);
    Ok(())
  }

  #[test]
  fn add_two_numbers_string_2() -> Result<(), CalculatorError> {
    assert_eq!(add("13,107")?, 120);
    Ok(())
  }

  #[test]
  fn add_two_invalid_string() -> Result<(), CalculatorError> {
    // TODO: Maybe add more specific parse error?
    assert!(matches!(add("35,bc"), Err(CalculatorError::ParseIntError(_))));
    Ok(())
  }

  #[test]
  fn add_three_numbers_string() -> Result<(), CalculatorError> {
    assert_eq!(add("11,209,1")?, 221);
    Ok(())
  }

  #[test]
  fn add_three_invalid_string() -> Result<(), CalculatorError> {
    assert!(matches!(add("11,209,1a"), Err(CalculatorError::ParseIntError(_))));
    Ok(())
  }

  #[test]
  fn add_five_numbers_string() -> Result<(), CalculatorError> {
    assert_eq!(add("1,0,1,0,1000")?, 1002);
    Ok(())
  }

  #[test]
  fn add_three_numbers_with_new_line_delimiter_string() -> Result<(), CalculatorError> {
    assert_eq!(add("1\n2,3")?, 6);
    Ok(())
  }

  #[test]
  fn add_one_with_invalid_delimiter_string() -> Result<(), CalculatorError> {
    // TODO: Maybe add more specific parse error for delimiter?
    assert!(matches!(add("1,\n"), Err(CalculatorError::ParseIntError(_))));
    Ok(())
  }

  #[test]
  fn add_four_numbers_with_new_line_delimiter_string() -> Result<(), CalculatorError> {
    assert_eq!(add("1\n2,3\n100")?, 106);
    Ok(())
  }

  #[test]
  fn add_two_numbers_with_semi_colon_delimiter_string() -> Result<(), CalculatorError> {
    assert_eq!(add("//;\n1;2")?, 3);
    Ok(())
  }

  #[test]
  fn add_three_numbers_with_star_delimiter_string() -> Result<(), CalculatorError> {
    assert_eq!(add("//*\n11*209*1")?, 221);
    Ok(())
  }
}
