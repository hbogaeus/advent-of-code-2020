
use regex::Regex;

#[derive(Debug, PartialEq)]
struct DatabaseEntry<'a> {
  low: i32,
  high: i32,
  lookup: char,
  password: &'a str
}

fn count_valid_passwords(input: Vec<String>) -> i32 {
  input.iter()
    .map(|x| parse_line(x))
    .filter(|x| validate_password(x))
    .count() as i32
}

fn validate_password(entry: &DatabaseEntry) -> bool {
  let mut counter = 0;

  for x in entry.password.chars() {
    if x == entry.lookup {
      counter = counter + 1;
    }
  }

  return counter >= entry.low && counter <= entry.high;
}

fn count_valid_passwords_two(input: Vec<String>) -> i32 {
  input.iter()
    .map(|x| parse_line(x))
    .filter(|x| validate_password_two(x))
    .count() as i32
}

fn validate_password_two(entry: &DatabaseEntry) -> bool {
  let low_match = entry.password.chars().nth((entry.low - 1) as usize).unwrap() == entry.lookup;
  let high_match = entry.password.chars().nth((entry.high - 1) as usize).unwrap() == entry.lookup;
  
  low_match ^ high_match
}

fn parse_line(input: &str) -> DatabaseEntry {
  let re = Regex::new(r"^(?P<low>\d+)-(?P<high>\d+)\s(?P<lookup>[a-z]): (?P<password>\w+)$").unwrap();
  let captures = re.captures(input).expect("Bad capture?");
  let low: i32 = captures.name("low").unwrap().as_str().parse().expect("No low");
  let high: i32 = captures.name("high").unwrap().as_str().parse().expect("No high");
  let lookup = captures.name("lookup").expect("No lookup").as_str().chars().next().expect("No lookup");
  let password = captures.name("password").expect("No password").as_str();

  DatabaseEntry {
    low,
    high,
    lookup, 
    password
  }
}

#[cfg(test)]
mod tests {
  use crate::utils::read_str;

    use super::*;

  #[test]
  fn parse_line_test() {
    let input = "11-33 a: abcde";
    let expected = DatabaseEntry { low: 11, high: 33, lookup: 'a', password: "abcde" };
    let actual = parse_line(input);

    assert_eq!(actual, expected)
  }

  #[test]
  fn validate_password_test_true() {
    let input = DatabaseEntry { low: 1, high: 3, lookup: 'a', password: "abcde" };
    let actual = validate_password(&input);
    assert_eq!(actual, true)
  }

  #[test]
  fn validate_password_test_false() {
    let input = DatabaseEntry { low: 1, high: 3, lookup: 'b', password: "cdefg" };
    let actual = validate_password(&input);
    assert_eq!(actual, false)
  }

  #[test]
  fn first_problem_test() {
    let input = vec![
      "1-3 a: abcde".into(),
      "1-3 b: cdefg".into(),
      "2-9 c: ccccccccc".into()
    ];

    let actual = count_valid_passwords(input);
    assert_eq!(actual, 2)
  }

  #[test]
  fn first_problem_test_input() {
    let input = read_str("src/day2/input.txt");
    let result = count_valid_passwords(input);
    println!("{}", result)
  }

  #[test]
  fn second_problem_test() {
    let input = DatabaseEntry { low: 1, high: 3, lookup: 'a', password: "abcde" };
    let actual = validate_password_two(&input);
    assert_eq!(actual, true)
  }

  #[test]
  fn second_problem_test_input() {
    let input = read_str("src/day2/input.txt");
    let result = count_valid_passwords_two(input);
    println!("{}", result)
  }
}