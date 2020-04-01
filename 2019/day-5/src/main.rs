use std::fs;
use std::io;

#[derive(PartialEq)]
enum OpCode {
    Add,
    Mult,
    Halt,
    Write,
    Read,
}

impl From<i32> for OpCode {
    fn from(item: i32) -> OpCode {
        use OpCode::*;

        match item {
            1 => Add,
            2 => Mult,
            3 => Write,
            4 => Read,
            99 => Halt,
            _ => unreachable!(),
        }
    }
}

fn main() -> std::io::Result<()> {
    let contents = fs::read_to_string("input.txt")?;

    let mut state: Vec<i32> = contents
        .split(',')
        .enumerate()
        .map(|(index, val)| {
            val.trim()
                .parse()
                .unwrap_or_else(|_| panic!("Input {} should an usize at {}", val, index))
        })
        .collect();

    exec_intcode(&mut state);

    Ok(())
}

fn exec_intcode(state: &mut Vec<i32>) {
    let mut index = 0;
    while index < state.len() {
        let instrc = state[index];
        let opcode = (instrc % 100).into();
        let params = get_param_modes(instrc)
            .iter()
            .enumerate()
            .map(|(param_num, val)| {
                let pos = state[index + param_num + 1];
                if val == &0 {
                    (state[pos as usize], pos)
                } else {
                    (pos, pos)
                }
            })
            .collect::<Vec<_>>();

        match opcode {
            OpCode::Add => state[params[2].1 as usize] = params[0].0 + params[1].0,
            OpCode::Mult => state[params[2].1 as usize] = params[0].0 * params[1].0,
            OpCode::Write => {
                let mut input = String::new();
                println!("Please provide an input:");
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");

                state[params[0].1 as usize] =
                    input.trim().parse().expect("Expected an integer input");
            }
            OpCode::Read => println!("{}", params[0].0),
            OpCode::Halt => break,
        }

        index += params.len() + 1; // include the opcode length offset
    }
}

fn get_param_modes(instruc: i32) -> Vec<u8> {
    let mut res = Vec::<u8>::new();

    // Number of parameters of each opcode
    let opcode = instruc % 100;
    let len = match opcode.into() {
        OpCode::Add => 3,
        OpCode::Mult => 3,
        OpCode::Write => 1,
        OpCode::Read => 1,
        OpCode::Halt => 0,
    };

    let mut modes = instruc / 100;
    for _ in 0..len {
        res.push((modes % 10) as u8);
        modes /= 10;
    }
    res
}
