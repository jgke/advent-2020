use itertools::izip;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

type Pos = (i32, i32, i32, i32);

fn neighbour_3d_pos((x, y, z, _w): Pos) -> Vec<Pos> {
    izip!(-1..=1, -1..=1, -1..=1)
        .filter(|(dx, dy, dz)| *dx != 0 || *dy != 0 || *dz != 0)
        .map(|(dx, dy, dz)| (x + dx, y + dy, z + dz, 0))
        .collect()
}

fn neighbour_4d_pos((x, y, z, w): Pos) -> Vec<Pos> {
    izip!(-1..=1, -1..=1, -1..=1, -1..=1)
        .filter(|(dx, dy, dz, dw)| *dx != 0 || *dy != 0 || *dz != 0 || *dw != 0)
        .map(|(dx, dy, dz, dw)| (x + dx, y + dy, z + dz, w + dw))
        .collect()
}

fn neighbours(pos: Pos, state: &HashSet<Pos>) -> usize {
    neighbour_4d_pos(pos)
        .into_iter()
        .filter(|pos| state.contains(&pos))
        .count()
}

fn step_1(state: &HashSet<Pos>) -> HashSet<Pos> {
    state
        .iter()
        .copied()
        .flat_map(|pos| neighbour_3d_pos(pos).into_iter())
        .filter_map(|pos| {
            let nbors = neighbours(pos, state);
            if (state.contains(&pos) && nbors >= 2 && nbors <= 3)
                || (!state.contains(&pos) && nbors == 3)
            {
                Some(pos)
            } else {
                None
            }
        })
        .collect()
}

fn part_1(initial_state: &HashSet<Pos>) -> usize {
    let mut state = initial_state.clone();
    for _ in 0..6 {
        state = step_1(&state);
    }
    state.len()
}

fn step_2(state: &HashSet<Pos>) -> HashSet<Pos> {
    state
        .iter()
        .copied()
        .flat_map(|pos| neighbour_4d_pos(pos).into_iter())
        .filter_map(|pos| {
            let nbors = neighbours(pos, state);
            if (state.contains(&pos) && nbors >= 2 && nbors <= 3)
                || (!state.contains(&pos) && nbors == 3)
            {
                Some(pos)
            } else {
                None
            }
        })
        .collect()
}

fn part_2(initial_state: &HashSet<Pos>) -> usize {
    let mut state = initial_state.clone();
    for _ in 0..6 {
        state = step_2(&state);
    }
    state.len()
}

fn parse<S: AsRef<str>>(lines: &[S]) -> HashSet<Pos> {
    lines
        .into_iter()
        .map(|s| {
            s.as_ref()
                .chars()
                .enumerate()
                .filter_map(|(x, c)| if c == '#' { Some(x) } else { None })
                .collect::<Vec<_>>()
        })
        .enumerate()
        .flat_map(|(y, line)| line.into_iter().map(move |x| (x as i32, y as i32, 0, 0)))
        .collect()
}

pub fn seventeen() -> Result<(), std::io::Error> {
    let file = File::open("17_input")?;
    let reader = BufReader::new(file);
    let lines = &reader.lines().map(|s| s.unwrap()).collect::<Vec<_>>();

    let state = parse(&lines);

    println!("Part 1: {}", part_1(&state));
    println!("Part 2: {}", part_2(&state));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_17::*;

    #[test]
    fn test() {
        let state = parse(&[".#.", "..#", "###"]);
        assert_eq!(112, part_1(&state));
    }

    #[test]
    fn test_2() {
        let state = parse(&[".#.", "..#", "###"]);
        assert_eq!(848, part_2(&state));
    }
}
