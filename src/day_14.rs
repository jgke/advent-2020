use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
enum Instruction {
    Mask(Vec<Option<bool>>),
    Mem(u64, u64),
}

impl Instruction {
    fn new(s: &str) -> Instruction {
        match s.chars().skip(1).next().unwrap() {
            'a' => Instruction::Mask(
                s.split(" = ")
                    .skip(1)
                    .next()
                    .unwrap()
                    .chars()
                    .map(|c| match c {
                        '1' => Some(true),
                        '0' => Some(false),
                        'X' => None,
                        _ => unreachable!(),
                    })
                    .collect(),
            ),
            'e' => {
                let mut s1 = s.split("] = ");
                let n1 = s1
                    .next()
                    .unwrap()
                    .split("[")
                    .skip(1)
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap();
                let n2 = s1.next().unwrap().parse().unwrap();

                Instruction::Mem(n1, n2)
            }
            _ => unreachable!(),
        }
    }
}

fn parse<S: AsRef<str>>(input: &[S]) -> Vec<Instruction> {
    input
        .into_iter()
        .map(|s| Instruction::new(s.as_ref()))
        .collect()
}

fn as_bits(n: u64) -> Vec<bool> {
    format!("{:#038b}", n)
        .chars()
        .skip(2)
        .map(|c| match c {
            '0' => false,
            '1' => true,
            _ => unreachable!(),
        })
        .collect()
}

fn from_bits(n: Vec<bool>) -> u64 {
    let mut res = 0;
    for bit in n {
        res <<= 1;
        res |= if bit { 1 } else { 0 };
    }
    res
}

fn apply_mask(value: &[bool], mask: &[Option<bool>]) -> Vec<bool> {
    value
        .iter()
        .zip(mask.iter())
        .map(|(v, mask)| mask.unwrap_or(*v))
        .collect()
}

fn part_1(instructions: &[Instruction]) -> u64 {
    let mut mask: Vec<Option<bool>> = (0..36).into_iter().map(|_| None).collect();
    let mut memory: HashMap<u64, u64> = HashMap::new();

    for instr in instructions {
        match instr {
            Instruction::Mask(m) => mask = m.to_vec(),
            Instruction::Mem(location, value) => {
                let write_val = from_bits(apply_mask(&as_bits(*value), &mask));
                memory.insert(*location, write_val);
            }
        }
    }

    memory.values().sum()
}

fn gen_masks(start: usize, address: &[bool], mask: &mut [Option<bool>]) -> Vec<Vec<bool>> {
    let mut res = Vec::new();
    for i in start..mask.len() {
        match mask[i] {
            None => {
                mask[i] = Some(true);
                res.append(&mut gen_masks(i + 1, address, mask));
                mask[i] = Some(false);
                res.append(&mut gen_masks(i + 1, address, mask));
                mask[i] = None;
                break;
            }
            Some(false) => {
                mask[i] = Some(address[i]);
            }
            Some(true) => {}
        }
    }
    if mask.iter().all(|s| s.is_some()) {
        vec![mask.iter().map(|s| s.unwrap()).collect()]
    } else {
        res
    }
}

fn part_2(instructions: &[Instruction]) -> u64 {
    let mut mask: Vec<Option<bool>> = (0..36).into_iter().map(|_| None).collect();
    let mut memory: HashMap<u64, u64> = HashMap::new();

    for instr in instructions {
        match instr {
            Instruction::Mask(m) => mask = m.to_vec(),
            Instruction::Mem(location, value) => {
                let mut mask_buf = mask.clone();
                for addr_mask in gen_masks(0, &as_bits(*location), &mut mask_buf) {
                    let address = from_bits(addr_mask);
                    memory.insert(address, *value);
                }
            }
        }
    }

    memory.values().sum()
}

pub fn fourteen() -> Result<(), std::io::Error> {
    let file = File::open("14_input")?;
    let reader = BufReader::new(file);
    let lines: Vec<Instruction> = parse(&reader.lines().map(|s| s.unwrap()).collect::<Vec<_>>());

    println!("Part 1: {}", part_1(&lines));
    println!("Part 2: {}", part_2(&lines));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_14::*;

    #[test]
    fn test() {
        let instrs = parse(&vec![
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
            "mem[8] = 11",
            "mem[7] = 101",
            "mem[8] = 0",
        ]);

        assert_eq!(165, part_1(&instrs));
    }

    #[test]
    fn test_2() {
        let instrs = parse(&vec![
            "mask = 000000000000000000000000000000X1001X",
            "mem[42] = 100",
            "mask = 00000000000000000000000000000000X0XX",
            "mem[26] = 1",
        ]);

        assert_eq!(208, part_2(&instrs));
    }
}
