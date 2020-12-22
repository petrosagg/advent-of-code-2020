use crate::lib::get_input;
use crate::lib::get_input_filter;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
enum Op {
    Mask(u64, u64),
    Set(u64, u64),
}


pub fn first() {
    let input = get_input(14, 1, |l| {
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

    let input = get_input(14, 0, |l| {
        let mut i = l.split(" = ");
        let op = i.next().unwrap();
        let arg = i.next().unwrap();
        match op {
            "mask" => {
                let or_mask = arg.replace('X', "0");
                let or_mask = u64::from_str_radix(&or_mask, 2).unwrap();
                let floats = arg.chars().enumerate().filter(|(_i, c)| *c == 'X').map(|(i, _)| 35 - i).collect::<Vec<_>>();
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
                
                let mut over = 0;
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
                            a |= 1 << position;
                        }
                    }
                    if memory.insert(a, value).is_some() {
                        over += 1;
                    }
                }
                // println!("overwritten {} values", over);
            }
        }
    }

    let sum = memory.values().fold(0, |acc, n| acc + n);
    dbg!(sum);
}

#[derive(Clone,Copy)]
struct AddrSpace (Option<(u64, u64)>);

impl AddrSpace {
    fn intersect(&self, other: Self) -> Self {
        match (self.0, other.0) {
            (Some((addr_a, mask_a)), Some((addr_b, mask_b))) => {
                let addr_a = addr_a & !(mask_b);
                let addr_b = addr_b & !(mask_a);
                if addr_a == addr_b {
                    Self(Some((addr_a, mask_a & mask_b)))
                } else {
                    Self(None)
                }
            }
            // Intersection with an empty set is the empty set
            _ => Self(None)
        }
    }

    fn size(&self) -> u64 {
        match self.0 {
            Some((_, mask)) => 1 << mask.count_ones(),
            None => 0
        }
    }
}

impl fmt::Debug for AddrSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some((addr, mask)) => {
                f.write_fmt(format_args!("addr={:036b} mask={:036b}", addr, mask))
            }
            None => {
                f.write_str("<empty>")
            }
        }
    }
}


pub fn second_new() {
    let assignments = get_input_filter(14, 0, {
        let mut cur_float_mask = 0;
        let mut cur_or_mask = 0;

        move |l| {
            let mut i = l.split(" = ");
            let op = i.next().unwrap();
            let arg = i.next().unwrap();
            match op {
                "mask" => {
                    let or_mask = arg.replace('X', "0");
                    cur_or_mask = u64::from_str_radix(&or_mask, 2).unwrap();

                    let float_mask = arg.replace('1', "0").replace('X', "1");
                    cur_float_mask = u64::from_str_radix(&float_mask, 2).unwrap();
                    None
                }
                _ => {
                    let (_, addr) = op.split_at(4);
                    let addr = addr[..addr.len()-1].parse::<u64>().unwrap();
                    let addr = (addr | cur_or_mask) & !(cur_float_mask);

                    let value = arg.parse::<u64>().unwrap();
                    Some((AddrSpace(Some((addr, cur_float_mask))), value))
                }
            }
        }
    });

    let mut acc = 0;
    for (a, value) in assignments {
        acc += a.size() * value;
        for (b, value) in prev_assignments {


        }
    }
    dbg!(assignments);

    // dbg!(acc);
}
