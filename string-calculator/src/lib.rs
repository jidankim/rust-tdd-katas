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


/// Method that can take two numbers, separated by commas, and will return their sum
fn add(numbers: &str) -> CalculatorResult {
  if numbers.is_empty() {
    Ok(0)
  } else {
    // TODO: Check if collect() call in the middle makes this code less performant
    Ok(numbers.split(",").map(|s| s.parse::<Number>()).collect::<Result<Vec<Number>, ParseIntError>>()?.iter().fold(0, |acc, num| acc + num))
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
}