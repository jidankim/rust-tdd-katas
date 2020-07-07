use std::num::ParseIntError;

type Number = i64;
type CalculatorResult = Result<Number, CalculatorError>;

#[derive(Debug)]
enum CalculatorError {
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
    // TODO: Check if collect() call in the middle makes this code less performant
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
}
