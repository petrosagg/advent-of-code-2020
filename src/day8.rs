use crate::lib::get_input;

#[derive(Debug,Clone)]
enum Instruction {
    Nop,
    Acc(i64),
    Jump(i64),
}

fn parse_instruction(l: &str) -> Instruction {
    let (ins, arg) = l.split_at(3);
    let arg = arg[1..].parse::<i64>().unwrap();
    match ins {
        "nop" => Instruction::Nop,
        "acc" => Instruction::Acc(arg),
        "jmp" => Instruction::Jump(arg),
        _ => panic!(ins.to_string()),
    }
}

pub fn first() {
    let instructions = get_input(8, 1, parse_instruction);

    let mut seen = vec![false; instructions.len()];
    let mut acc = 0;
    let mut pc = 0;

    loop {
        if seen[pc as usize] {
            println!("{}", acc);
            break;
        }
        seen[pc as usize] = true;
        match instructions[pc as usize] {
            Instruction::Nop => pc += 1,
            Instruction::Acc(n) => {
                acc += n;
                pc += 1;
            },
            Instruction::Jump(n) => pc += n,
        }
    }
}

pub fn second() {
    let mut instructions = get_input(8, 1, parse_instruction);


    for i in 0..instructions.len() {
        let original = instructions[i].clone();
        match original {
            Instruction::Nop => instructions[i] = Instruction::Jump(0),
            Instruction::Jump(_) => instructions[i] = Instruction::Nop,
            _ => continue,
        }

        let mut seen = vec![false; instructions.len()];
        let mut acc = 0;
        let mut pc: i64 = 0;
        loop {
            if pc as usize == instructions.len() {
                println!("{}", acc);
                return;
            }
            if seen[pc as usize] {
                break;
            }
            seen[pc as usize] = true;
            match instructions[pc as usize] {
                Instruction::Nop => pc += 1,
                Instruction::Acc(n) => {
                    acc += n;
                    pc += 1;
                },
                Instruction::Jump(n) => pc += n,
            }
        }

        instructions[i] = original;
    }
}
