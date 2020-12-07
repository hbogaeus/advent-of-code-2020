use std::{fmt::{Debug, Display}, fs::File, io::{BufRead, BufReader}};

pub fn print_grid<T: Display>(input: Vec<Vec<T>>) {
  for row in input {
    for cell in row {
      print!("{} ", cell)
    }
    print!("\n");
  }
}

fn read_from_file<F, T>(filename: &str, f: F) -> Vec<T>  where F: Fn(&str) -> T {
  let file = File::open(filename).expect("File not found");
  let br = BufReader::new(file);

  let mut v: Vec<T> = Vec::new();

  for line in br.lines() {
    let line = line.expect("Broken line");

    let value = f(line.trim());

    v.push(value);
  }

  v
}

fn read_lines_from_file(filename: &str) -> impl Iterator<Item = String> {
  let file = File::open(filename).expect("File not found");
  let br = BufReader::new(file);

  br.lines()
    .map(|line| line.expect("Broken line in input"))
    .map(|line| line.trim().to_string())
}

pub fn read_integers(filename: &str) -> Vec<i32> {
  read_from_file(filename, |x| x.parse().expect("Bad number"))
}

pub fn read_str(filename: &str) -> Vec<String> {
  read_from_file(filename, |x| x.to_string())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_grid_print() {
    let input = vec![
      vec![1, 2, 3],
      vec![4, 5, 6],
      vec![7, 8, 9],
    ];

    print_grid(input);
  }
}