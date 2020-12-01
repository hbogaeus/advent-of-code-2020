use std::{fs::File, io::{BufRead, BufReader}};

pub fn read_integers(filename: &str) -> Vec<i32> {
  let file = File::open(filename).expect("File not found");
  let br = BufReader::new(file);

  let mut v: Vec<i32> = Vec::new();

  for line in br.lines() {
    let line = line.expect("Broken line");

    let n: i32  = line.trim()
      .parse()
      .expect("not a number");

    v.push(n);
  }

  v
}