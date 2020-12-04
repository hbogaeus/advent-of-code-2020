use std::{fs::File, io::{BufRead, BufReader}};

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

pub fn read_integers(filename: &str) -> Vec<i32> {
  read_from_file(filename, |x| x.parse().expect("Bad number"))
}

pub fn read_str(filename: &str) -> Vec<String> {
  read_from_file(filename, |x| x.to_string())
}