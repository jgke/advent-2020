use crate::grid::Grid;
use std::fmt;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Clone, Debug, PartialEq)]
enum Cell {
    Dead,
    Alive,
    Floor,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Cell::Dead => '#',
            Cell::Alive => 'L',
            Cell::Floor => '.',
        };
        write!(f, "{}", c)
    }
}

impl Cell {
    fn new(c: char) -> Cell {
        match c {
            'L' => Cell::Dead,
            '#' => Cell::Alive,
            '.' => Cell::Floor,
            _ => panic!(),
        }
    }

    fn num(&self) -> u32 {
        match self {
            Cell::Dead => 0,
            Cell::Alive => 1,
            Cell::Floor => 0,
        }
    }

    fn modify(&mut self, neighbours: u32, alive_limit: u32) -> bool {
        match self {
            Cell::Floor => false,
            Cell::Dead => {
                if neighbours == 0 {
                    *self = Cell::Alive;
                }
                neighbours == 0
            }
            Cell::Alive => {
                if neighbours >= alive_limit {
                    *self = Cell::Dead;
                }
                neighbours >= 5
            }
        }
    }
}

fn neighbours_1(grid: &Grid<Cell>, x: i32, y: i32) -> u32 {
    let mut sum = 0;
    for a in (y - 1)..=(y + 1) {
        if a < 0 {
            continue;
        }
        if let Some(row) = grid.elems.get(a as usize) {
            for b in (x - 1)..=(x + 1) {
                if b < 0 {
                    continue;
                }
                if let Some(cell) = row.get(b as usize) {
                    sum += cell.num();
                }
            }
        }
    }
    sum
}

fn step_1(grid: &Grid<Cell>) -> (bool, Grid<Cell>) {
    let mut new_grid = grid.clone();
    let mut modified = false;
    for y in 0..grid.col_size() {
        for x in 0..grid.row_size() {
            let neigbours = neighbours_1(&grid, x as i32, y as i32);
            modified |= new_grid.elems[y][x].modify(neigbours, 5);
        }
    }
    (modified, new_grid)
}

fn neighbours_2(grid: &Grid<Cell>, x: i32, y: i32) -> u32 {
    let mut sum = 0;

    for dx in -1..=1 {
        for dy in -1..=1 {
            sum += grid
                .ray(x, y, dx, dy, |c| c == &Cell::Floor)
                .unwrap_or(&Cell::Floor)
                .num();
        }
    }

    sum
}

fn step_2(grid: &Grid<Cell>) -> (bool, Grid<Cell>) {
    let mut new_grid = grid.clone();
    let mut modified = false;
    for y in 0..grid.col_size() {
        for x in 0..grid.row_size() {
            let neigbours = neighbours_2(&grid, x as i32, y as i32);
            modified |= new_grid.elems[y][x].modify(neigbours, 5);
        }
    }
    (modified, new_grid)
}

fn parse<T: AsRef<str>>(lines: &[T]) -> Grid<Cell> {
    let lines = lines
        .iter()
        .map(|s| s.as_ref().chars().map(Cell::new).collect())
        .collect();
    Grid::new(lines)
}

pub fn eleven() -> Result<(), std::io::Error> {
    let file = File::open("11_input")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|s| s.unwrap()).collect();

    let mut grid = parse(&lines);

    loop {
        let (modified, new_grid) = step_1(&grid);
        grid = new_grid;
        if !modified {
            break;
        }
    }

    println!(
        "Part 1: {}",
        grid.elems
            .iter()
            .map(|row| row.iter().map(|e| e.num()).sum::<u32>())
            .sum::<u32>()
    );

    let mut grid = parse(&lines);

    loop {
        let (modified, new_grid) = step_2(&grid);
        grid = new_grid;
        if !modified {
            break;
        }
    }

    println!(
        "Part 2: {}",
        grid.elems
            .iter()
            .map(|row| row.iter().map(|e| e.num()).sum::<u32>())
            .sum::<u32>()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_11::*;

    #[test]
    fn grid_count() {
        let grid = parse(&vec!["#.#", "L.L", ".##"]);

        let nbors: Vec<Vec<u32>> = (-1..=3)
            .into_iter()
            .map(|y| {
                (-1..=3)
                    .into_iter()
                    .map(|x| neighbours_1(&grid, x, y))
                    .collect()
            })
            .collect();

        // .....
        // .#.#.
        // .L.L.
        // ..##.
        // .....

        assert_eq!(
            vec![
                vec![1, 1, 2, 1, 1],
                vec![1, 1, 2, 1, 1],
                vec![1, 2, 4, 3, 2],
                vec![0, 1, 2, 2, 1],
                vec![0, 1, 2, 2, 1],
            ],
            nbors
        );

        let big_grid = parse(&vec![
            "#.##.##.##",
            "#######.##",
            "#.#.#..#..",
            "####.##.##",
            "#.##.##.##",
            "#.#####.##",
            "..#.#.....",
            "##########",
            "#.######.#",
            "#.#####.##",
        ]);

        assert_eq!(4, neighbours_1(&big_grid, 6, 0));
    }

    #[track_caller]
    fn modify(expected: Cell, mut cell: Cell, nbor: u32) {
        let expected_change = cell != expected;
        assert_eq!(expected_change, cell.modify(nbor, 5));
        assert_eq!(expected, cell);
    }

    #[test]
    fn mod_test() {
        for i in 0..=8 {
            modify(Cell::Floor, Cell::Floor, i);
        }

        modify(Cell::Alive, Cell::Dead, 0);
        for i in 1..=8 {
            modify(Cell::Dead, Cell::Dead, i);
        }

        for i in 1..=4 {
            modify(Cell::Alive, Cell::Alive, i);
        }
        for i in 5..=8 {
            modify(Cell::Dead, Cell::Alive, i);
        }
    }

    #[test]
    fn generations() {
        let mut grid = parse(&[
            "L.LL.LL.LL",
            "LLLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLLL",
            "L.LLLLLL.L",
            "L.LLLLL.LL",
        ]);

        grid = step_1(&grid).1;

        assert_eq!(
            parse(&[
                "#.##.##.##",
                "#######.##",
                "#.#.#..#..",
                "####.##.##",
                "#.##.##.##",
                "#.#####.##",
                "..#.#.....",
                "##########",
                "#.######.#",
                "#.#####.##"
            ]),
            grid
        );

        grid = step_1(&grid).1;

        assert_eq!(
            parse(&[
                "#.LL.L#.##",
                "#LLLLLL.L#",
                "L.L.L..L..",
                "#LLL.LL.L#",
                "#.LL.LL.LL",
                "#.LLLL#.##",
                "..L.L.....",
                "#LLLLLLLL#",
                "#.LLLLLL.L",
                "#.#LLLL.##"
            ]),
            grid
        );
    }

    #[test]
    fn grid_count_2() {
        let grid = parse(&vec!["#.#", "L.L", ".##"]);

        let nbors: Vec<Vec<u32>> = (-1..=3)
            .into_iter()
            .map(|y| {
                (-1..=3)
                    .into_iter()
                    .map(|x| neighbours_2(&grid, x, y))
                    .collect()
            })
            .collect();

        // .....
        // .#.#.
        // .L.L.
        // ..##.
        // .....

        assert_eq!(
            vec![
                vec![1, 1, 3, 1, 1],
                vec![1, 2, 3, 1, 1],
                vec![1, 2, 4, 3, 2],
                vec![1, 2, 1, 2, 1],
                vec![1, 1, 2, 2, 1],
            ],
            nbors
        );
    }
}
