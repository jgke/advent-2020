use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Entry {
    min: usize,
    max: usize,
    chr: char,
    password: String,
}

impl Entry {
    fn from_str(from: &str) -> Option<Entry> {
        let parts = from
            .split(" ")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let limits = parts.get(0)?.split("-").collect::<Vec<_>>();
        let chr = parts.get(1)?.chars().next()?;
        let password = parts.get(2)?.to_string();
        Some(Entry {
            min: limits[0].parse().unwrap(),
            max: limits[1].parse().unwrap(),
            chr,
            password,
        })
    }

    fn valid_1(&self) -> bool {
        let count = self.password.chars().filter(|c| *c == self.chr).count();
        count >= self.min && count <= self.max
    }

    fn valid_2(&self) -> bool {
        let chrs = self.password.chars().collect::<Vec<_>>();
        (chrs[self.min - 1] == self.chr) ^ (chrs[self.max - 1] == self.chr)
    }
}

pub fn two() -> Result<(), std::io::Error> {
    let file = File::open("2_input")?;
    let reader = BufReader::new(file);
    let lines: Vec<Entry> = reader
        .lines()
        .map(|s| Entry::from_str(&s.unwrap()).unwrap())
        .collect();

    // part 1
    println!(
        "Valid passwords: {}",
        lines.iter().filter(|e| e.valid_1()).count()
    );

    // part 2
    println!(
        "Valid passwords: {}",
        lines.iter().filter(|e| e.valid_2()).count()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_2::Entry;
    #[test]
    fn it_works() {
        let entries = vec![
            Entry::from_str("1-3 a: abcde").unwrap(),
            Entry::from_str("1-3 b: cdefg").unwrap(),
            Entry::from_str("2-9 c: ccccccccc").unwrap(),
        ];
        assert_eq!(true, entries[0].valid_1());
        assert_eq!(false, entries[1].valid_1());
        assert_eq!(true, entries[2].valid_1());

        assert_eq!(true, entries[0].valid_2());
        assert_eq!(false, entries[1].valid_2());
        assert_eq!(false, entries[2].valid_2());
    }
}
