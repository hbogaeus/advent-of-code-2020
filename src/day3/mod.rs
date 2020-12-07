struct Slope {
    x: usize,
    y: usize,
}

fn count_trees_multi_slopes(input: &Vec<String>, slopes: Vec<Slope>) -> i64 {
    slopes
        .iter()
        .map(|slope| count_trees(&input, slope))
        .map(|value| {
            println!("{}", value);
            value as i64
        })
        .product()
}

fn count_trees(input: &Vec<String>, slope: &Slope) -> i32 {
    let height = input.len();
    let width = input[0].len();

    let mut grid: Vec<Vec<char>> = Vec::new();

    for x in input.iter() {
        let mut row = Vec::new();

        for c in x.chars() {
            row.push(c);
        }
        grid.push(row);
    }

    let mut x = slope.x;
    let mut y = slope.y;

    let mut tree_count = 0;

    while y < height {
        let square = grid[y][x];

        if square == '#' {
            tree_count += 1;
            grid[y][x] = 'X';
        } else {
            grid[y][x] = 'O';
        }

        x = (x + slope.x) % width;
        y = y + slope.y;
    }

    tree_count
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::utils::read_str;

    use super::*;

    #[test]
    fn first_problem_test() {
        let input = vec![
            "..##.......".into(),
            "#...#...#..".into(),
            ".#....#..#.".into(),
            "..#.#...#.#".into(),
            ".#...##..#.".into(),
            "..#.##.....".into(),
            ".#.#.#....#".into(),
            ".#........#".into(),
            "#.##...#...".into(),
            "#...##....#".into(),
            ".#..#...#.#".into(),
        ];

        let count = count_trees(&input, &Slope { x: 3, y: 1 });

        assert_eq!(count, 7);
    }

    #[test]
    fn first_problem_test_input() {
        let input = read_str("src/day3/input.txt");
        let count = count_trees(&input, &Slope { x: 3, y: 1 });

        println!("{}", count);
    }

    #[test]
    fn second_problem_test() {
        let input: Vec<String> = vec![
            "..##.......".into(),
            "#...#...#..".into(),
            ".#....#..#.".into(),
            "..#.#...#.#".into(),
            ".#...##..#.".into(),
            "..#.##.....".into(),
            ".#.#.#....#".into(),
            ".#........#".into(),
            "#.##...#...".into(),
            "#...##....#".into(),
            ".#..#...#.#".into(),
        ];

        let slopes = vec![
            Slope { x: 1, y: 1 },
            Slope { x: 3, y: 1 },
            Slope { x: 5, y: 1 },
            Slope { x: 7, y: 1 },
            Slope { x: 1, y: 2 },
        ];

        let count = count_trees_multi_slopes(&input, slopes);

        assert_eq!(count, 336);
    }

    #[test]
    fn second_problem_test_input() {
        let input = read_str("src/day3/input.txt");

        let slopes = vec![
            Slope { x: 1, y: 1 },
            Slope { x: 3, y: 1 },
            Slope { x: 5, y: 1 },
            Slope { x: 7, y: 1 },
            Slope { x: 1, y: 2 },
        ];

        let count = count_trees_multi_slopes(&input, slopes);
        println!("{}", count)
    }
}
