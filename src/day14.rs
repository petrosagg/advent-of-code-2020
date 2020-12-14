use crate::lib::get_input;
use std::collections::HashMap;

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

pub fn second_new() {
    #[derive(Debug)]
    enum Op {
        Mask(u64, u64),
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

                let float_mask = arg.replace('1', "0").replace('X', "1");
                let float_mask = u64::from_str_radix(&float_mask, 2).unwrap();
                Op::Mask(or_mask, float_mask)
            }
            _ => {
                let (_, addr) = op.split_at(4);
                let addr = addr[..addr.len()-1].parse::<u64>().unwrap();
                let arg = arg.parse::<u64>().unwrap();
                Op::Set(addr, arg)
            }
        }
    });

    let mut operations = vec![];
    let mut cur_float = 0;
    let mut cur_or_mask = 0;
    let mut acc = 0;
    for op in input {
        match op {
            Op::Mask(or, float) => {
                cur_or_mask = or;
                cur_float = float;
            },
            Op::Set(addr, value) => {
                // First, apply the current OR mask and zero out floating bits
                let addr = (addr | cur_or_mask) & !(cur_float);
                println!("\naddr: {:036b}", addr);
                println!("floa: {:036b}", cur_float);

                // Now we have to walk the operations backward, cancelling any that overlap. As we
                // walk backwards, we adjust the effective float mask to know what can still be
                // affected by previous operations
                let mut effective_float = cur_float;
                for (prev_addr, prev_float, prev_value) in operations.iter().rev() {
                    // The only way to have overlap is for the addresses after zeroing the floating
                    // bits to equal
                    println!("\n\tprocessing prev value {}", prev_value);
                    println!("\tprev_addr float:  {:036b}", prev_float);
                    println!("\taddr & !prev:     {:036b}", addr & !(prev_float));
                    println!("\tprev_addr & !eff: {:036b}", prev_addr & !(effective_float));
                    if (addr & !(prev_float)) == prev_addr & !(effective_float) {
                        // Ok, so now we need to figure out how much of the previous operation we
                        // need to undo. This is the overlap or their masks
                        let overlap = effective_float & prev_float;
                        println!("\tremoving {} * {}", (1 << overlap.count_ones()), prev_value);
                        acc -= (1 << overlap.count_ones()) * prev_value;

                        // We leave unaffected floating bits to maybe cancel other operations
                        effective_float = effective_float & !(prev_float);

                        // We cancelled everything there is to cancel
                        if effective_float == 0 {
                            break;
                        }
                    }
                }

                // Finally, add the current value for all the possible addresses. We do this last
                // to avoid possible overflowing
                println!("\tadding {} * {}", (1 << cur_float.count_ones()), value);
                acc += (1 << cur_float.count_ones()) * value;

                operations.push((addr, cur_float, value));
            }
        }
    }

    dbg!(acc);
}
