use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Passport {
    parts: HashMap<String, String>,
}

pub fn valid_byr(val: &str) -> bool {
    let year = val.parse::<u32>().unwrap_or(0);
    year >= 1920 && year <= 2002
}

pub fn valid_iyr(val: &str) -> bool {
    let year = val.parse::<u32>().unwrap_or(0);
    year >= 2010 && year <= 2020
}

pub fn valid_eyr(val: &str) -> bool {
    let year = val.parse::<u32>().unwrap_or(0);
    year >= 2020 && year <= 2030
}

pub fn valid_hgt(val: &str) -> bool {
    let regex = Regex::new("([0-9]+)(cm|in)").unwrap();
    for cap in regex.captures_iter(val) {
        let height = cap[1].parse::<u32>().unwrap_or(0);
        let unit = &cap[2] == "cm";
        if unit {
            return height >= 150 && height <= 193;
        } else {
            return height >= 59 && height <= 76;
        }
    }
    false
}

pub fn valid_hcl(val: &str) -> bool {
    Regex::new("^#[0-9a-f]{6}$").unwrap().is_match(val)
}

pub fn valid_ecl(val: &str) -> bool {
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&val)
}

pub fn valid_pid(val: &str) -> bool {
    Regex::new("^[0-9]{9}$").unwrap().is_match(val)
}

impl Passport {
    fn from_str(from: &str) -> Passport {
        let parts = from
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| {
                let mut split = s.split(":");
                let key = split.next().unwrap();
                let value = split.next().unwrap();
                (key.to_string(), value.to_string())
            })
            .collect::<HashMap<_, _>>();
        Passport { parts }
    }

    fn valid_1(&self) -> bool {
        for key in &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"] {
            if !self.parts.contains_key(*key) {
                return false;
            }
        }
        return true;
    }

    fn valid_2(&self) -> bool {
        let fields: Vec<(&str, fn(&str) -> bool)> = vec![
            ("byr", valid_byr),
            ("iyr", valid_iyr),
            ("eyr", valid_eyr),
            ("hgt", valid_hgt),
            ("hcl", valid_hcl),
            ("ecl", valid_ecl),
            ("pid", valid_pid),
        ];

        for (key, validator) in &fields {
            if let Some(val) = self.parts.get(*key) {
                if !validator(val) {
                    return false;
                }
            }
        }
        true
    }
}

pub fn split_input(lines: Vec<String>) -> Vec<String> {
    let mut grouped = Vec::new();
    let mut groups = Vec::new();
    for line in lines {
        if line.is_empty() && grouped.len() > 0 {
            groups.push(grouped);
            grouped = Vec::new();
        } else {
            grouped.push(line);
        }
    }

    if grouped.len() > 0 {
        groups.push(grouped);
    }

    groups.into_iter().map(|s| s.join(" ")).collect()
}

pub fn four() -> Result<(), std::io::Error> {
    let file = File::open("4_input")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|s| s.unwrap()).collect();
    let mut passports = Vec::new();

    for group in split_input(lines) {
        passports.push(Passport::from_str(&group));
    }

    // part 1
    println!(
        "Valid passports: {}",
        passports.iter().filter(|p| p.valid_1()).count()
    );
    // part 2
    println!(
        "Valid passports: {}",
        passports
            .iter()
            .filter(|p| p.valid_1() && p.valid_2())
            .count()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_4::*;
    #[test]
    fn part_1() {
        let input = "
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
";

        let split = split_input(input.lines().map(|s| s.to_string()).collect());

        assert_eq!(
            vec![
                " ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm",
                "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929",
                "hcl:#ae17e1 iyr:2013 eyr:2024 ecl:brn pid:760753108 byr:1931 hgt:179cm",
                "hcl:#cfa07d eyr:2025 pid:166559648 iyr:2011 ecl:brn hgt:59in"
            ],
            split
        );

        assert_eq!(
            vec![true, false, true, false],
            split
                .into_iter()
                .map(|s| Passport::from_str(&s))
                .map(|p| p.valid_1())
                .collect::<Vec<_>>()
        )
    }

    #[test]
    fn validators() {
        assert!(!valid_byr("1919"));
        assert!(valid_byr("1920"));
        assert!(valid_byr("2002"));
        assert!(!valid_byr("2003"));

        assert!(!valid_iyr("2009"));
        assert!(valid_iyr("2010"));
        assert!(valid_iyr("2020"));
        assert!(!valid_iyr("2021"));

        assert!(!valid_eyr("2019"));
        assert!(valid_eyr("2020"));
        assert!(valid_eyr("2030"));
        assert!(!valid_eyr("2031"));

        assert!(!valid_hgt("58in"));
        assert!(valid_hgt("59in"));
        assert!(valid_hgt("76in"));
        assert!(!valid_hgt("77in"));

        assert!(!valid_hgt("149cm"));
        assert!(valid_hgt("150cm"));
        assert!(valid_hgt("193cm"));
        assert!(!valid_hgt("194cm"));

        assert!(valid_hgt("190cm"));
        assert!(!valid_hgt("190in"));
        assert!(!valid_hgt("190"));

        assert!(valid_hcl("#123abc"));
        assert!(!valid_hcl("#123abcd"));
        assert!(!valid_hcl("#123abz"));
        assert!(!valid_hcl("123abc"));

        assert!(valid_ecl("brn"));
        assert!(!valid_ecl("brne"));
        assert!(!valid_ecl("wat"));

        assert!(valid_pid("000000001"));
        assert!(!valid_pid("0123456789"));
    }

    #[test]
    fn part_2() {
        let invalids = "
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
";
        for line in split_input(invalids.lines().map(|s| s.to_string()).collect()) {
            let passport = Passport::from_str(&line);
            assert!(!passport.valid_2(), line);
        }

        let valids = "
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
";
        for line in split_input(valids.lines().map(|s| s.to_string()).collect()) {
            let passport = Passport::from_str(&line);
            assert!(passport.valid_2(), line);
        }
    }
}
