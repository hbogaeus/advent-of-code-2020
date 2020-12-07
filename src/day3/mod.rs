use ndarray::Array2;

use crate::utils::print_grid;

fn travel(input: Vec<&str>) {
  let height = input.len();
  let width = input[0].len();

  let mut grid = Array2::from_elem((width, height), ' ');

  for x in input {
    for (index, y) in x.chars().enumerate() {
      grid[[]]
    }
  }

  let mut x = 0;
  let mut y = 0;

  let mut tree_count = 0;

  print_grid(grid);

  while y < height {
    let square = grid[x][y];

    if grid[x][y] == Square::Tree {
      tree_count += 1;
    }

    x = x + 1;
    y = y + 3;
  }

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn testing() {
    let input = vec![
      "..##.......",
      "#...#...#..",
      ".#....#..#.",
      "..#.#...#.#",
      ".#...##..#.",
      "..#.##.....",
      ".#.#.#....#",
      ".#........#",
      "#.##...#...",
      "#...##....#",
      ".#..#...#.#"
    ];

    travel(input)
  }
}