fn find_solution_one(input: Vec<i32>) -> Option<i32> {
  for x in 0..input.len() {
    for y in x+1..input.len() {
      if input[x] + input[y] == 2020 {
        let answer = input[x] * input[y];
        println!("Values are {} and {} and answer is {}", input[x], input[y], answer);
        return Some(answer);
      }
    }
  }

  return None;
}

fn find_solution_two(input: Vec<i32>) -> Option<i32> {
  for x in 0..input.len() {
    for y in x+1..input.len() {
      for z in x+1..input.len() {
        if input[x] + input[y] + input[z] == 2020 {
          let answer = input[x] * input[y] * input[z];
          println!("Values are {} and {} and {} and answer is {}", input[x], input[y], input[z], answer);
          return Some(answer);
          }
        }
      }
    }

  return None;
}


#[cfg(test)]
mod tests {
  use crate::utils::read_integers;
  use super::*;

  #[test]
  fn first_problem_test() {
    let input = vec![
      1721,
      979,
      366,
      299,
      675,
      1456
    ];

    let result = find_solution_one(input);
    assert_eq!(result.unwrap(), 514579)
  }

  #[test]
  fn first_problem_input_test() {
    let input = read_integers("src/day1/input.txt");
    find_solution_one(input);
  }

  #[test]
  fn second_problem_test() {
    let input = vec![
      1721,
      979,
      366,
      299,
      675,
      1456
    ];

    let result = find_solution_two(input);
    assert_eq!(result.unwrap(), 241861950)
  }

  #[test]
  fn second_problem_input_test() {
    let input = read_integers("src/day1/input.txt");
    find_solution_two(input);
  }
}