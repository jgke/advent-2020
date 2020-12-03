use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use crate::grid::Grid;

#[derive(PartialEq, Eq)]
pub enum Cell {
    Tree,
    Empty,
}

pub fn count_trees(grid: &Grid<Cell>, start: usize, dx: usize, dy: usize) -> usize {
    let mut count = 0;
    let row_size = grid.row_size();
    let mut x = start;
    let mut y = 0;

    while y < grid.col_size() {
        if grid.elems[y][x % row_size] == Cell::Tree {
            count += 1;
        }
        y += dy;
        x += dx;
    }

    count
}

pub fn v1(lines: &Grid<Cell>) -> usize {
    count_trees(lines, 0, 3, 1)
}

pub fn v2(lines: &Grid<Cell>) -> usize {
    count_trees(lines, 0, 1, 1)
        * count_trees(lines, 0, 3, 1)
        * count_trees(lines, 0, 5, 1)
        * count_trees(lines, 0, 7, 1)
        * count_trees(lines, 0, 1, 2)
}

pub fn parse_line(line: &str) -> Vec<Cell> {
    line.chars()
        .map(|c| if c == '#' { Cell::Tree } else { Cell::Empty })
        .collect()
}

pub fn three() -> Result<(), std::io::Error> {
    let file = File::open("3_input")?;
    let reader = BufReader::new(file);
    let lines: Grid<Cell> = Grid::new(reader.lines().map(|s| parse_line(&s.unwrap())).collect());

    println!("Part 1: {}", v1(&lines));
    println!("Part 2: {}", v2(&lines));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_3::*;
    #[test]
    fn it_works() {
        let lines = vec![
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
            ".#..#...#.#",
        ];
        let input = Grid::new(lines.into_iter().map(parse_line).collect());
        assert_eq!(7, v1(&input));
        assert_eq!(336, v2(&input));
    }
}
