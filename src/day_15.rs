use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn simulator(lines: &[usize], turn_limit: usize) -> usize {
    let mut memory: HashMap<usize, (usize, Option<usize>)> = HashMap::new();
    let mut prev = 0;
    let mut turn = 0;

    for line in lines {
        turn += 1;
        memory.insert(*line, (turn, None));
        prev = *line;
    }

    loop {
        if turn == turn_limit {
            return prev;
        }

        turn += 1;

        let next = if let (_, Some(diff_to_prev_pos)) = memory[&prev] {
            diff_to_prev_pos
        } else {
            0
        };

        match memory.get(&next).copied() {
            None => {
                memory.insert(next, (turn, None));
            }
            Some((prev_pos, _)) => {
                memory.insert(next, (turn, Some(turn - prev_pos)));
            }
        }

        prev = next;
    }
}

pub fn fifteen() -> Result<(), std::io::Error> {
    let file = File::open("15_input")?;
    let reader = BufReader::new(file);
    let lines: Vec<usize> = reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    println!("Part 1: {}", simulator(&lines, 2020));
    println!("Part 2: {}", simulator(&lines, 30000000));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_15::*;

    #[test]
    fn test() {
        assert_eq!(436, simulator(&[0, 3, 6], 2020));
        assert_eq!(175594, simulator(&[0, 3, 6], 30000000));
    }
}
