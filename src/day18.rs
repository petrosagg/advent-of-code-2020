use itertools::Itertools;

use crate::lib::*;

#[derive(Debug,PartialEq,Clone,Copy)]
enum Token {
    LParen,
    RParen,
    Add,
    Mul,
    Num(u64),
}

fn eval_simple<I: Iterator<Item=Token>>(tokens: &mut I, depth: usize) -> u64 {
    let mut acc = 0;
    let mut op = None;

    while let Some(token) = tokens.next() {
        // println!("{:width$}acc={} op={:?} token={:?}", ' ', acc, op, token, width=depth*4);
        match (token, op) {
            (o @ Token::Add, _) | (o @ Token::Mul, _) => {
                op = Some(o);
            },
            (Token::Num(n), None) => {
                acc = n;
            }
            (Token::Num(n), Some(Token::Add)) => {
                acc += n;
            },
            (Token::Num(n), Some(Token::Mul)) => {
                acc *= n;
            },
            (Token::LParen, None) => {
                acc = eval_simple(tokens, depth + 1);
            }
            (Token::LParen, Some(Token::Add)) => {
                let n = eval_simple(tokens, depth + 1);
                acc += n;
            },
            (Token::LParen, Some(Token::Mul)) => {
                let n = eval_simple(tokens, depth + 1);
                acc *= n;
            },
            (Token::RParen, _) => {
                break;
            }
            _ => panic!(),
        }
    }
    acc
}


pub fn first() {
    let exprs = get_input(18, 1, |l| {
        l.split(' ').map(|token| match token {
            "(" => Token::LParen,
            ")" => Token::RParen,
            "*" => Token::Mul,
            "+" => Token::Add,
            _ => Token::Num(token.parse().unwrap()),
        }).collect_vec()
    });

    let mut sum = 0;
    for expr in exprs {
        sum += eval_simple(&mut expr.into_iter(), 0);
    }
    dbg!(sum);
}

fn eval_advanced<I: Iterator<Item=Token>>(tokens: &mut I, depth: usize) -> (u64, bool) {
    let mut acc = 0;
    let mut op = None;
    let mut should_break = false;

    while let Some(token) = tokens.next() {
        // println!("{:width$}acc={} op={:?} token={:?}", ' ', acc, op, token, width=depth*4);
        match (token, op) {
            (o @ Token::Add, None) => {
                op = Some(o);
            },
            (Token::Mul, None) => {
                let (n, should_break) = eval_advanced(tokens, depth + 1);
                acc *= n;
                if should_break {
                    return (acc, should_break);
                }
            },
            (Token::Num(n), None) => {
                assert!(acc == 0);
                acc = n;
            }
            (Token::Num(n), Some(Token::Add)) => {
                acc += n;
                op = None;
            },
            (Token::LParen, None) => {
                assert!(acc == 0);
                let (n, _) = eval_advanced(tokens, depth + 1);
                acc = n;
            }
            (Token::LParen, Some(Token::Add)) => {
                let (n, _) = eval_advanced(tokens, depth + 1);
                acc += n;
                op = None;
            },
            (Token::RParen, _) => {
                should_break = true;
                break;
            }
            _ => panic!(),
        }
    }
    (acc, should_break)
}

pub fn second() {
    let exprs = get_input(18, 1, |l| {
        let l = l.replace("(", "( ").replace(")", " )");
        l.split(' ').map(|token| match token {
            "(" => Token::LParen,
            ")" => Token::RParen,
            "*" => Token::Mul,
            "+" => Token::Add,
            _ => Token::Num(token.parse().unwrap()),
        }).collect_vec()
    });

    let mut acc = 0;
    for expr in exprs {
        acc += eval_advanced(&mut expr.into_iter(), 0).0;
    }
    dbg!(acc);
}
