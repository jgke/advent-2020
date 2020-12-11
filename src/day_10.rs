use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn part_1(lines: &Vec<i32>) -> (i32, i32, i32) {
    let mut diff_1 = 0;
    let mut diff_2 = 0;
    let mut diff_3 = 0;
    let mut prev = 0;

    for line in lines {
        if line - prev == 1 {
            diff_1 += 1;
        } else if line - prev == 2 {
            diff_2 += 1;
        } else if line - prev == 3 {
            diff_3 += 1;
        }
        prev = *line;
    }

    (diff_1, diff_2, diff_3)
}

pub fn recur(
    map: &HashMap<i32, Vec<i32>>,
    times: &mut HashMap<i32, i64>,
    this: i32,
    target: i32,
) -> i64 {
    if this == target {
        return 1;
    }
    if times.contains_key(&this) {
        return times[&this];
    }
    let mut counts = 0;
    for child in &map[&this] {
        counts += recur(map, times, *child, target);
    }
    times.insert(this, counts);
    counts
}

pub fn part_2(lines: &Vec<i32>) -> i64 {
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();

    for line in lines {
        map.insert(*line, Vec::new());
    }

    for line in lines {
        if map.contains_key(&(line + 1)) {
            map.get_mut(&line).unwrap().push(line + 1);
        }
        if map.contains_key(&(line + 2)) {
            map.get_mut(&line).unwrap().push(line + 2);
        }
        if map.contains_key(&(line + 3)) {
            map.get_mut(&line).unwrap().push(line + 3);
        }
    }

    recur(&map, &mut HashMap::new(), 0, *lines.iter().max().unwrap())
}

pub fn ten() -> Result<(), std::io::Error> {
    let file = File::open("10_input")?;
    let reader = BufReader::new(file);
    let mut lines: Vec<i32> = reader
        .lines()
        .map(|s| s.unwrap().parse().unwrap())
        .collect();
    lines.push(0);
    lines.push(lines.iter().max().unwrap() + 3);
    lines.sort();

    let p1 = part_1(&lines);

    println!("Part 1: {}", p1.0 * p1.2);
    println!("Part 2: {}", part_2(&lines));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_10::*;

    #[test]
    fn test() {
        let mut input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        input.push(0);
        input.push(input.iter().max().unwrap() + 3);
        input.sort();
        assert_eq!((7, 0, 5), part_1(&input));
        assert_eq!(8, part_2(&input));
    }

    #[test]
    fn test_2() {
        let mut input = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        input.push(0);
        input.push(input.iter().max().unwrap() + 3);
        input.sort();
        assert_eq!((22, 0, 10), part_1(&input));
        assert_eq!(19208, part_2(&input));
    }
}
