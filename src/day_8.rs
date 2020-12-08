use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl Instruction {
    pub fn new(from: &str) -> Instruction {
        let mut split = from.split(" ");
        let instr = split.next().unwrap();
        let count = split.next().unwrap().parse().unwrap();
        match instr {
            "nop" => Instruction::Nop(count),
            "acc" => Instruction::Acc(count),
            "jmp" => Instruction::Jmp(count),
            _ => panic!(),
        }
    }
}

pub fn simulate(instrs: &Vec<Instruction>) -> Result<i32, i32> {
    let mut acc = 0;
    let mut ins = 0;

    let mut visited = HashSet::new();
    visited.insert(0);

    loop {
        match instrs.get(ins) {
            Some(Instruction::Nop(_)) => {
                ins += 1;
            }
            Some(Instruction::Acc(count)) => {
                acc += count;
                ins += 1;
            }
            Some(Instruction::Jmp(pos)) => {
                ins = (ins as i32 + pos) as usize;
            }
            None => return Err(acc),
        }

        if visited.contains(&ins) {
            break;
        }
        visited.insert(ins);
    }

    Ok(acc)
}

pub fn loop_simulator(mut instrs: Vec<Instruction>) -> Result<i32, i32> {
    for i in 0..instrs.len() {
        match instrs[i] {
            Instruction::Nop(pos) => {
                instrs[i] = Instruction::Jmp(pos);
                simulate(&instrs)?;
                instrs[i] = Instruction::Nop(pos);
            }
            Instruction::Jmp(pos) => {
                instrs[i] = Instruction::Nop(pos);
                simulate(&instrs)?;
                instrs[i] = Instruction::Jmp(pos);
            }
            _ => {}
        }
    }

    panic!()
}

pub fn eight() -> Result<(), std::io::Error> {
    let file = File::open("8_input")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|s| s.unwrap()).collect();

    let instrs = lines
        .iter()
        .map(|s| Instruction::new(&s))
        .collect::<Vec<_>>();

    println!("Part 1: {}", simulate(&instrs).unwrap());
    println!("Part 2: {}", loop_simulator(instrs).unwrap_err());

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_8::Instruction::*;
    use crate::day_8::*;

    #[test]
    fn ops() {
        let ops = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        let parsed = ops
            .lines()
            .map(|line| Instruction::new(line))
            .collect::<Vec<_>>();
        assert_eq!(
            vec![
                Nop(0),
                Acc(1),
                Jmp(4),
                Acc(3),
                Jmp(-3),
                Acc(-99),
                Acc(1),
                Jmp(-4),
                Acc(6)
            ],
            parsed
        );

        let part_1 = simulate(&parsed).unwrap();
        assert_eq!(5, part_1);

        let part_2 = loop_simulator(parsed).unwrap_err();
        assert_eq!(8, part_2);
    }
}
