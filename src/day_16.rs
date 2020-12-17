use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug, PartialEq)]
struct Validator {
    name: String,
    values: HashSet<usize>,
}

fn parse_validator(line: &str) -> Validator {
    let mut parts = line.split(": ");
    let name = parts.next().unwrap();
    let values = parts
        .next()
        .unwrap()
        .split(" or ")
        .map(|range| {
            let mut v = range.split("-");
            let left = v.next().unwrap().parse::<usize>().unwrap();
            let right = v.next().unwrap().parse::<usize>().unwrap();
            left..=right
        })
        .flatten()
        .collect();
    Validator {
        name: name.to_string(),
        values,
    }
}

fn parse_ticket(line: &str) -> Vec<usize> {
    line.split(",").map(|f| f.parse().unwrap()).collect()
}

fn parse(s: &str) -> (Vec<Validator>, Vec<usize>, Vec<Vec<usize>>) {
    let mut groups = s.split("\n\n");

    let validators = groups
        .next()
        .unwrap()
        .lines()
        .map(parse_validator)
        .collect();
    let your = parse_ticket(groups.next().unwrap().lines().skip(1).next().unwrap());
    let others = groups
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(parse_ticket)
        .collect();

    (validators, your, others)
}

fn possibly_valid(validators: &[Validator], ticket: &Vec<usize>) -> Option<usize> {
    for field in ticket {
        if validators
            .iter()
            .all(|validator| !validator.values.contains(field))
        {
            return Some(*field);
        }
    }
    None
}

fn part_1(validators: &[Validator], tickets: &Vec<Vec<usize>>) -> usize {
    tickets
        .iter()
        .map(|ticket| possibly_valid(validators, ticket))
        .flatten()
        .sum()
}

fn search_path(
    field: usize,
    paths: &[Vec<String>],
    visited: &mut HashSet<String>,
) -> Option<Vec<String>> {
    if field == paths.len() {
        return Some(Vec::new());
    }
    for possibility in &paths[field] {
        if !visited.contains(possibility) {
            visited.insert(possibility.to_string());
            if let Some(mut solution) = search_path(field + 1, paths, visited) {
                solution.push(possibility.to_string());
                return Some(solution);
            }
            visited.remove(possibility);
        }
    }
    None
}

fn part_2(
    validators: &[Validator],
    your: &[usize],
    tickets: &Vec<Vec<usize>>,
) -> HashMap<String, usize> {
    let validators: HashMap<String, HashSet<usize>> = validators
        .iter()
        .map(|Validator { name, values }| (name.to_string(), values.clone()))
        .collect();

    let mut possibilities: Vec<Vec<String>> = Vec::new();
    for i in 0..tickets[0].len() {
        let mut poss = Vec::new();
        'next_validator: for (validator, values) in &validators {
            for ticket in tickets {
                if !values.contains(&ticket[i]) {
                    continue 'next_validator;
                }
            }
            poss.push(validator.to_string());
        }
        possibilities.push(poss);
    }
    let mut path = search_path(0, &possibilities, &mut HashSet::new()).unwrap();
    path.reverse();
    path.into_iter()
        .enumerate()
        .map(|(i, v)| (v, your[i]))
        .collect()
}

pub fn sixteen() -> Result<(), std::io::Error> {
    let file = File::open("16_input")?;
    let reader = BufReader::new(file);
    let (validators, your, others) = parse(
        &reader
            .lines()
            .map(|s| s.unwrap())
            .collect::<Vec<_>>()
            .join("\n"),
    );

    println!("Part 1: {}", part_1(&validators, &others));

    let only_valid_others = others
        .into_iter()
        .filter(|ticket| possibly_valid(&validators, ticket).is_none())
        .collect::<Vec<_>>();

    println!(
        "Part 2: {}",
        part_2(&validators, &your, &only_valid_others)
            .iter()
            .filter_map(|(k, v)| if k.starts_with("departure") {
                Some(v)
            } else {
                None
            })
            .fold(1, |a, b| a * b)
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_16::*;

    #[test]
    fn test() {
        let (validators, _your, others) = parse(
            "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12",
        );
        assert_eq!(71, part_1(&validators, &others));
    }

    #[test]
    fn test_2() {
        let (validators, your, others) = parse(
            "class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9",
        );

        let mut expected = HashMap::new();
        expected.insert("class".to_string(), 12);
        expected.insert("row".to_string(), 11);
        expected.insert("seat".to_string(), 13);

        assert_eq!(expected, part_2(&validators, &your, &others));
    }
}
