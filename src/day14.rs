use crate::lib::get_input;
use std::collections::HashMap;

#[derive(Debug)]
enum Op {
    Mask(u64, u64),
    Set(u64, u64),
}

pub fn first() {
    let mut input = get_input(14, 1, |l| {
        let mut i = l.split(" = ");
        let op = i.next().unwrap();
        let arg = i.next().unwrap();
        match op {
            "mask" => {
                let and_mask = arg.replace('X', "1");
                let or_mask = arg.replace('X', "0");
                let and_mask = u64::from_str_radix(&and_mask, 2).unwrap();
                let or_mask = u64::from_str_radix(&or_mask, 2).unwrap();
                Op::Mask(and_mask, or_mask)
            }
            _ => {
                let (_, addr) = op.split_at(4);
                let addr = addr[..addr.len()-1].parse::<u64>().unwrap();
                let arg = arg.parse::<u64>().unwrap();
                Op::Set(addr, arg)
            }
        }
    });

    let mut memory = HashMap::new();

    let mut cur_and_mask = 0;
    let mut cur_or_mask = 0;
    for op in input {
        match op {
            Op::Mask(and, or) => {
                cur_and_mask = and;
                cur_or_mask = or;
            },
            Op::Set(addr, value) => {
                memory.insert(addr, (value & cur_and_mask) | cur_or_mask);
            }
        }
    }

    let sum = memory.values().fold(0, |acc, n| acc + n);

    dbg!(sum);
}

pub fn second() {
    #[derive(Debug)]
    enum Op {
        Mask(u64, Vec<usize>),
        Set(u64, u64),
    }

    let mut input = get_input(14, 1, |l| {
        let mut i = l.split(" = ");
        let op = i.next().unwrap();
        let arg = i.next().unwrap();
        match op {
            "mask" => {
                let or_mask = arg.replace('X', "0");
                let or_mask = u64::from_str_radix(&or_mask, 2).unwrap();
                let floats = arg.chars().enumerate().filter(|(i, c)| *c == 'X').map(|(i, _)| 35 - i).collect::<Vec<_>>();
                Op::Mask(or_mask, floats)
            }
            _ => {
                let (_, addr) = op.split_at(4);
                let addr = addr[..addr.len()-1].parse::<u64>().unwrap();
                let arg = arg.parse::<u64>().unwrap();
                Op::Set(addr, arg)
            }
        }
    });

    let mut memory = HashMap::new();

    let mut cur_floats = vec![];
    let mut cur_or_mask = 0;
    for op in input {
        match op {
            Op::Mask(or, floats) => {
                cur_floats = floats;
                cur_or_mask = or;
            },
            Op::Set(addr, value) => {
                let addr = addr | cur_or_mask;
                
                let f = 1 << cur_floats.len();
                for i in 0..f {
                    let mut a = addr;
                    for (j, position) in cur_floats.iter().enumerate() {
                        // Take the current bit
                        let bit = (i >> (cur_floats.len() - j - 1)) & 1;

                        // Put it in the right position
                        if bit == 0 {
                            // we need to unset it
                            a &= !(1 << position);
                        } else {
                            // we need to set it
                            a |= (1 << position);
                        }
                    }
                    memory.insert(a, value);
                }
            }
        }
    }

    let sum = memory.values().fold(0, |acc, n| acc + n);
    dbg!(sum);
}
