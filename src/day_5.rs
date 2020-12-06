use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn passport_number(input: &str) -> (usize, usize) {
    let mut row = 0;
    let mut col = 0;
    let vec = input.chars().collect::<Vec<_>>();
    let mut iter = vec.chunks(7);
    let first = iter.next().unwrap();
    let second = iter.next().unwrap();

    for c in first {
        row <<= 1;
        if *c == 'B' {
            row += 1;
        }
    }
    for c in second {
        col <<= 1;
        if *c == 'R' {
            col += 1;
        }
    }
    (row, col)
}

pub fn five() -> Result<(), std::io::Error> {
    let file = File::open("5_input")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|s| s.unwrap()).collect();

    let mut seats = (1..1024).collect::<HashSet<usize>>();

    let mut numbers = lines
        .iter()
        .map(|s| passport_number(&s))
        .map(|(row, col)| row * 8 + col)
        .collect::<Vec<_>>();
    for num in &numbers {
        seats.remove(num);
    }

    println!("Highest number: {}", numbers.iter().max().unwrap());

    let mut sorted_nums = seats.into_iter().collect::<Vec<_>>();
    sorted_nums.sort();

    println!("Missing seats:");
    for n in sorted_nums {
        println!("{}", n);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_5::*;
    #[test]

    fn passports() {
        assert_eq!(passport_number("FBFBBFFRLR"), (44, 5));
        assert_eq!(passport_number("BFFFBBFRRR"), (70, 7));
        assert_eq!(passport_number("FFFBBBFRRR"), (14, 7));
        assert_eq!(passport_number("BBFFBBFRLL"), (102, 4));
    }
}
