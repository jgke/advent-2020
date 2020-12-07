use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn bag_contents(input: &str) -> (String, Vec<(String, usize)>) {
    let mut iter = input.split(" ");

    let this = format!("{} {}", iter.next().unwrap(), iter.next().unwrap());
    let mut content = Vec::new();

    iter.next(); //bags
    iter.next(); //contains

    loop {
        let num = iter.next();
        if num == None || num == Some("no") {
            break;
        }

        let color = format!("{} {}", iter.next().unwrap(), iter.next().unwrap());
        iter.next(); // bags

        content.push((color, num.unwrap().parse().unwrap()));
    }

    return (this, content);
}

fn recur(
    color: &str,
    map: &HashMap<String, Vec<(String, usize)>>,
    can: &mut HashSet<String>,
    cannot: &mut HashSet<String>,
) -> bool {
    if can.contains(color) || color == "shiny gold" {
        return true;
    } else if cannot.contains(color) {
        return false;
    }

    for (inner_color, _) in &map[color] {
        if recur(inner_color, map, can, cannot) {
            can.insert(color.to_string());
            return true;
        }
    }

    cannot.insert(color.to_string());
    return false;
}

pub fn one(map: &HashMap<String, Vec<(String, usize)>>) -> usize {
    let mut can_contain = HashSet::new();
    let mut cannot_contain = HashSet::new();

    for color in map.keys() {
        recur(color, map, &mut can_contain, &mut cannot_contain);
    }

    can_contain.len()
}

fn recur_two(
    color: &str,
    map: &HashMap<String, Vec<(String, usize)>>,
    count: &mut HashMap<String, usize>,
) -> usize {
    if count.contains_key(color) {
        return count[color];
    }

    let mut n = 0;

    for (inner_color, times) in &map[color] {
        n += times * (1 + recur_two(inner_color, map, count));
    }

    count.insert(color.to_string(), n);
    return n;
}

pub fn two(map: &HashMap<String, Vec<(String, usize)>>) -> usize {
    let mut count = HashMap::new();

    recur_two("shiny gold", map, &mut count)
}

pub fn seven() -> Result<(), std::io::Error> {
    let file = File::open("7_input")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|s| s.unwrap()).collect();

    let map = lines
        .iter()
        .map(|s| bag_contents(&s))
        .collect::<HashMap<_, _>>();

    println!("Bags containing shiny gold: {}", one(&map));
    println!("Shiny gold contains {} bags", two(&map));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_7::*;

    fn eq(expected: (&str, Vec<(&str, usize)>), input: &str) {
        let result = bag_contents(input);
        assert_eq!(result.0, expected.0);
        assert_eq!(
            result.1,
            expected
                .1
                .into_iter()
                .map(|(s, v)| (s.to_string(), v))
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn parse() {
        eq(
            ("light red", vec![("bright white", 1), ("muted yellow", 2)]),
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
        );
        eq(
            (
                "dark orange",
                vec![("bright white", 3), ("muted yellow", 4)],
            ),
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
        );
        eq(
            ("bright white", vec![("shiny gold", 1)]),
            "bright white bags contain 1 shiny gold bag.",
        );
        eq(
            ("muted yellow", vec![("shiny gold", 2), ("faded blue", 9)]),
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
        );
        eq(
            ("shiny gold", vec![("dark olive", 1), ("vibrant plum", 2)]),
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
        );
        eq(
            ("dark olive", vec![("faded blue", 3), ("dotted black", 4)]),
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
        );
        eq(
            ("vibrant plum", vec![("faded blue", 5), ("dotted black", 6)]),
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
        );
        eq(
            ("faded blue", vec![]),
            "faded blue bags contain no other bags.",
        );
        eq(
            ("dotted black", vec![]),
            "dotted black bags contain no other bags.",
        );
    }

    #[test]
    fn counts() {
        let rules = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        let map = rules
            .lines()
            .map(|s| bag_contents(&s))
            .collect::<HashMap<_, _>>();

        assert_eq!(4, one(&map));
        assert_eq!(32, two(&map));
    }
}
