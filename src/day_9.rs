use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn calculator(preamble: usize, lines: &Vec<i64>) -> i64 {
    let mut sets = Vec::new();

    for i in 0..lines.len() {
        sets.push(HashSet::new());
        let left = if i > preamble { i - preamble } else { 0 };
        for h in left..i {
            sets[h].insert(lines[i] + lines[h]);
        }

        if i >= preamble {
            let needle = lines[i];
            let mut found = false;
            for h in (i - preamble)..i {
                if sets[h].contains(&needle) {
                    found = true;
                    break;
                }
            }
            if !found {
                return needle;
            }
        }
    }

    panic!();
}

pub fn part_2(lines: &Vec<i64>, needle: i64) -> i64 {
    for start in 0..lines.len() {
        let mut sum = needle;
        for h in start..lines.len() {
            sum -= lines[h];
            if sum == 0 {
                let min = lines[start..=h].iter().min().unwrap();
                let max = lines[start..=h].iter().max().unwrap();
                return min + max;
            } else if sum < 0 {
                break;
            }
        }
    }

    panic!()
}

pub fn nine() -> Result<(), std::io::Error> {
    let file = File::open("9_input")?;
    let reader = BufReader::new(file);
    let lines: Vec<i64> = reader
        .lines()
        .map(|s| s.unwrap().parse().unwrap())
        .collect();

    let result = calculator(25, &lines);

    println!("Part 1: {}", result);
    println!("Part 2: {}", part_2(&lines, result));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_9::*;

    #[test]
    fn results() {
        let lines = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        let result = calculator(5, &lines);

        assert_eq!(127, result);
        assert_eq!(62, part_2(&lines, result));
    }
}
