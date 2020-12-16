use std::fmt;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Clone, Debug, PartialEq)]
enum Instruction {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Rotate((i32, i32), (i32, i32)),
    Forward(i32),
}

fn rot(n: i32) -> Instruction {
    match n {
        0 => Instruction::Rotate((1, 0), (0, 1)),
        90 => Instruction::Rotate((0, -1), (1, 0)),
        180 => Instruction::Rotate((-1, 0), (0, -1)),
        270 => Instruction::Rotate((0, 1), (-1, 0)),
        _ => unreachable!(),
    }
}

impl Instruction {
    fn new(s: &str) -> Instruction {
        let c = s.chars().next().unwrap();
        let n = s.chars().skip(1).collect::<String>().parse().unwrap();
        match c {
            'N' => Instruction::North(n),
            'S' => Instruction::South(n),
            'E' => Instruction::East(n),
            'W' => Instruction::West(n),
            'F' => Instruction::Forward(n),

            'L' => rot(n),
            'R' => rot((-n + 360) % 360),

            _ => unreachable!(),
        }
    }
}

fn part_1(lines: &[Instruction]) -> (i32, i32) {
    let mut position: (i32, i32) = (0, 0);
    let mut direction = (1, 0);

    for line in lines {
        match line {
            Instruction::North(n) => position.1 += n,
            Instruction::South(n) => position.1 -= n,
            Instruction::East(n) => position.0 += n,
            Instruction::West(n) => position.0 -= n,
            Instruction::Forward(n) => {
                position.0 += direction.0 * n;
                position.1 += direction.1 * n;
            }
            Instruction::Rotate((m1, m2), (m3, m4)) => {
                direction = (
                    m1 * direction.0 + m2 * direction.1,
                    m3 * direction.0 + m4 * direction.1,
                );
            }
        }
    }

    println!("Part 1: {}", position.0.abs() + position.1.abs());
    position
}

fn part_2(lines: &[Instruction]) -> (i32, i32) {
    let mut position: (i32, i32) = (0, 0);
    let mut direction = (10, 1);

    for line in lines {
        match line {
            Instruction::North(n) => direction.1 += n,
            Instruction::South(n) => direction.1 -= n,
            Instruction::East(n) => direction.0 += n,
            Instruction::West(n) => direction.0 -= n,
            Instruction::Forward(n) => {
                position.0 += direction.0 * n;
                position.1 += direction.1 * n;
            }
            Instruction::Rotate((m1, m2), (m3, m4)) => {
                direction = (
                    m1 * direction.0 + m2 * direction.1,
                    m3 * direction.0 + m4 * direction.1,
                );
            }
        }
    }

    println!("Part 2: {}", position.0.abs() + position.1.abs());

    position
}

pub fn twelve() -> Result<(), std::io::Error> {
    let file = File::open("12_input")?;
    let reader = BufReader::new(file);
    let lines: Vec<Instruction> = reader
        .lines()
        .map(|s| Instruction::new(&s.unwrap()))
        .collect();

    part_1(&lines);
    part_2(&lines);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_12::*;

    #[test]
    fn test() {
        let lines: Vec<Instruction> = ["F10", "N3", "F7", "R90", "F11"]
            .iter()
            .map(|s| Instruction::new(s))
            .collect();

        assert_eq!((17, -8), part_1(&lines));
        assert_eq!((214, -72), part_2(&lines));
    }
}
