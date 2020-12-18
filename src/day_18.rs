use std::collections::VecDeque;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug, PartialEq)]
enum Token {
    Number(i64),
    Plus,
    Times,
    OpenParen,
    CloseParen,
}

#[derive(Debug, PartialEq)]
enum Node {
    Number(i64),
    Plus(Box<Node>, Box<Node>),
    Times(Box<Node>, Box<Node>),
}

impl Token {
    fn new(s: char) -> Token {
        match s {
            '+' => Token::Plus,
            '*' => Token::Times,
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            _ => Token::Number(s.to_digit(10).unwrap() as i64),
        }
    }
}

fn tokenize(s: &str) -> Vec<Token> {
    s.chars()
        .filter(|c| *c != ' ')
        .map(|c| Token::new(c))
        .collect()
}

fn eval(node: Node) -> i64 {
    match node {
        Node::Number(n) => n,
        Node::Plus(a, b) => eval(*a) + eval(*b),
        Node::Times(a, b) => eval(*a) * eval(*b),
    }
}

fn parse_one_1(tokens: &mut VecDeque<Token>) -> Node {
    match tokens.pop_front().unwrap() {
        Token::Number(n) => Node::Number(n),
        Token::OpenParen => {
            let inner = parse(tokens);
            assert_eq!(Token::CloseParen, tokens.pop_front().unwrap());
            inner
        }
        other => panic!("Invalid token: {:?}", other),
    }
}

fn parse(tokens: &mut VecDeque<Token>) -> Node {
    let mut left = parse_one_1(tokens);
    loop {
        match tokens.front() {
            None | Some(Token::CloseParen) => break,
            Some(Token::Plus) => {
                assert_eq!(Token::Plus, tokens.pop_front().unwrap());
                left = Node::Plus(Box::new(left), Box::new(parse_one_1(tokens)));
            }
            Some(Token::Times) => {
                assert_eq!(Token::Times, tokens.pop_front().unwrap());
                left = Node::Times(Box::new(left), Box::new(parse_one_1(tokens)));
            }
            other => panic!("Invalid token: {:?}", other),
        }
    }
    left
}

fn calculate(s: &str) -> i64 {
    let mut tokens = tokenize(s).into_iter().collect();
    eval(parse(&mut tokens))
}

fn parse_2(tokens: &mut VecDeque<Token>, prec: bool) -> Node {
    let mut left = match tokens.pop_front().unwrap() {
        Token::Number(n) => Node::Number(n),
        Token::OpenParen => {
            let inner = parse_2(tokens, false);
            assert_eq!(Token::CloseParen, tokens.pop_front().unwrap());
            inner
        }
        other => panic!("Invalid token: {:?}", other),
    };
    loop {
        match tokens.front() {
            None | Some(Token::CloseParen) => break,
            Some(Token::Plus) => {
                assert_eq!(Token::Plus, tokens.pop_front().unwrap());
                left = Node::Plus(Box::new(left), Box::new(parse_2(tokens, true)));
            }
            Some(Token::Times) if prec => break,
            Some(Token::Times) => {
                assert_eq!(Token::Times, tokens.pop_front().unwrap());
                left = Node::Times(Box::new(left), Box::new(parse_2(tokens, false)));
            }
            other => panic!("Invalid token: {:?}", other),
        }
    }
    left
}

fn calculate_2(s: &str) -> i64 {
    let mut tokens = tokenize(s).into_iter().collect();
    eval(parse_2(&mut tokens, false))
}

pub fn eighteen() -> Result<(), std::io::Error> {
    let file = File::open("18_input")?;
    let reader = BufReader::new(file);
    let lines = &reader.lines().map(|s| s.unwrap()).collect::<Vec<_>>();

    println!(
        "Part 1: {}",
        lines.iter().map(|s| calculate(&s)).sum::<i64>()
    );
    println!(
        "Part 2: {}",
        lines.iter().map(|s| calculate_2(&s)).sum::<i64>()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_18::*;

    #[test]
    fn test() {
        assert_eq!(71, calculate("1 + 2 * 3 + 4 * 5 + 6"));
        assert_eq!(26, calculate("2 * 3 + (4 * 5)"));
        assert_eq!(437, calculate("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
        assert_eq!(
            12240,
            calculate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
        );
        assert_eq!(
            13632,
            calculate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
        );

        assert_eq!(51, calculate_2("1 + (2 * 3) + (4 * (5 + 6))"));
        assert_eq!(46, calculate_2("2 * 3 + (4 * 5)"));
        assert_eq!(1445, calculate_2("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
        assert_eq!(
            669060,
            calculate_2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
        );
        assert_eq!(
            23340,
            calculate_2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
        );
    }
}
