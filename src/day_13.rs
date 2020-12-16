use std::fmt;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn part_1(earliest_time: usize, times: &Vec<usize>) -> (usize, usize) {
    let mut current_time = earliest_time;
    loop {
        for time in times {
            if current_time % time == 0 {
                return (current_time - earliest_time, *time);
            }
        }
        current_time += 1;
    }
}

pub fn part_2(times: &Vec<(usize, usize)>) -> usize {
    let mut m = 0;
    let mut period = 1;

    for (i, time) in times.iter() {
        let mut k = 0;
        while (period * k + m + i) % *time != 0 {
            k += 1;
        }

        m += period * k;
        period *= time;
    }

    m
}

fn parse(s: &str) -> Vec<(usize, usize)> {
    s.split(",")
        .map(|s| s.parse())
        .enumerate()
        .filter_map(|(i, n)| n.ok().map(|n| (i, n)))
        .collect()
}

pub fn thirteen() -> Result<(), std::io::Error> {
    let file = File::open("13_input")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|s| s.unwrap()).collect();

    let earliest_time: usize = lines[0].parse().unwrap();
    let bus_times: Vec<usize> = lines[1].split(",").map(|s| s.parse()).flatten().collect();

    let p1 = part_1(earliest_time, &bus_times);
    println!("Part 1: {}", p1.0 * p1.1);

    let times: Vec<(usize, usize)> = parse(&lines[1]);
    println!("Part 2: {}", part_2(&times));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_13::*;

    #[test]
    fn test() {
        assert_eq!((5, 59), part_1(939, &vec![7, 13, 59, 31, 19]));
        assert_eq!(0, part_2(&parse("3")));
        assert_eq!(9, part_2(&parse("3,5")));
        assert_eq!(3417, part_2(&parse("17,x,13,19")));
        assert_eq!(754018, part_2(&parse("67,7,59,61")));
        assert_eq!(779210, part_2(&parse("67,x,7,59,61")));
        assert_eq!(1261476, part_2(&parse("67,7,x,59,61")));
        assert_eq!(1202161486, part_2(&parse("1789,37,47,1889")));
    }
}
