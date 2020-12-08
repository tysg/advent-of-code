#[derive(Debug, Clone, Copy)]
enum Op {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl From<&str> for Op {
    fn from(input: &str) -> Op {
        let num = input[4..].parse::<i32>().unwrap();
        match &input[..3] {
            "nop" => Op::Nop(num),
            "acc" => Op::Acc(num),
            "jmp" => Op::Jmp(num),
            _ => panic!("this op code is not recognized"),
        }
    }
}

pub fn solve(input: &str) {
    let mut instructions = input.lines().map(Op::from).collect::<Vec<_>>();

    if let Err(n) = run(&instructions) {
        println!("{}", n);
    }

    match try_fix_instructions(&mut instructions) {
        Some(res) => println!("{}", res),
        None => println!("not found"),
    }
    
}

fn try_fix_instructions(instr: &mut Vec<Op>) -> Option<i32> {
    // brute force all possibilities
    for i in 0..instr.len() {
        let saved_op = instr[i];
        // change i
        match &instr[i] {
            Op::Nop(n) => instr[i] = Op::Jmp(*n),
            Op::Jmp(n) => instr[i] = Op::Nop(*n),
            _ => ()
        }

        if let Ok(result) = run(&instr) {
            return Some(result);
        }
        instr[i] = saved_op;
    }

    None
}

fn run(instr: &Vec<Op>) -> Result<i32, i32> {
    let len = instr.len();
    let mut accum = 0;
    let mut visited = vec![false; len];
    let mut ptr = 0;

    loop {
        visited[ptr] = true;
        match &instr[ptr] {
            Op::Acc(n) => {
                accum += n;
                ptr += 1;
            }
            Op::Jmp(n) => {
                let next = ptr as i32 + n;
                if next < 0 {
                    return Err(accum);
                } else {
                    ptr = next as usize;
                }
            }
            Op::Nop(_) => {
                ptr += 1;
            }
        }

        if ptr == len {
            return Ok(accum);
        }

        if visited[ptr] || ptr > len {
            return Err(accum);
        }
    }
}
