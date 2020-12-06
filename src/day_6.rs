use crate::utils::as_groups;

use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn six() -> Result<(), std::io::Error> {
    let file = File::open("6_input")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|s| s.unwrap()).collect();

    let mut sum = 0;

    let chunks = as_groups(lines);

    let orig_group = ('a'..='z').collect::<HashSet<_>>();

    for chunk in chunks.clone() {
        let mut group = orig_group.clone();
        for line in chunk {
            for c in line.chars() {
                group.remove(&c);
            }
        }
        sum += orig_group.len() - group.len();
    }

    println!("Sum: {}", sum);

    let mut new_sum = 0;

    for chunk in chunks {
        let mut group = orig_group.clone();
        for line in chunk {
            group = group
                .intersection(&line.chars().collect())
                .map(|x| *x)
                .collect();
        }
        new_sum += group.len();
    }

    println!("New sum: {}", new_sum);

    Ok(())
}
