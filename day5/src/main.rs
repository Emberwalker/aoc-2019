use std::env;
use std::io;
use std::io::Write;

mod types;
use types::*;

fn main() {
    // Call with --part-two to calculate part 2 answer
    let _part_two = env::args()
        .nth(1)
        .map_or(false, |arg| "--part-two".eq(&arg));

    let mut raw_program = String::new();
    print!("Enter Intcode string: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut raw_program)
        .expect("failed to get Intcode from terminal");

    let mut segments: Vec<i32> = Vec::new();
    for str_segment in raw_program.split(',') {
        let trimmed_segment = str_segment.trim();
        let segment: i32 = trimmed_segment
            .parse()
            .unwrap_or_else(|_| panic!("failed to parse segment {}", trimmed_segment));
        segments.push(segment);
    }

    let end_pos = execute(&mut segments);
    println!("Program END at {}: {:?}", end_pos, segments);
}

fn execute(segments: &mut Vec<i32>) -> usize {
    let mut idx: usize = 0;
    loop {
        let instr: Instruction = segments[idx].into();
        match instr.opcode {
            Opcode::Add => op_add(segments, &mut idx, instr.modes),
            Opcode::Mul => op_mul(segments, &mut idx, instr.modes),
            Opcode::Read => op_read(segments, &mut idx, instr.modes),
            Opcode::Write => op_write(segments, &mut idx, instr.modes),
            Opcode::JumpIfTrue => op_jump_if_true(segments, &mut idx, instr.modes),
            Opcode::JumpIfFalse => op_jump_if_false(segments, &mut idx, instr.modes),
            Opcode::LessThan => op_less_than(segments, &mut idx, instr.modes),
            Opcode::Equals => op_equals(segments, &mut idx, instr.modes),
            Opcode::End => break,
        }
    }
    idx
}

fn yank(segments: &[i32], idx: usize, mode: Mode) -> i32 {
    match mode {
        Mode::Position => segments[segments[idx] as usize],
        Mode::Immediate => segments[idx],
    }
}

fn out_address(segments: &[i32], idx: usize, mode: Mode) -> usize {
    match mode {
        Mode::Position => yank(segments, idx, Mode::Immediate) as usize,
        Mode::Immediate => idx,
    }
}

fn get_binary_op_chunk(
    segments: &[i32],
    opcode_idx: usize,
    Modes(op1_mode, op2_mode, _): Modes,
) -> (i32, i32, usize) {
    let operand_one = yank(segments, opcode_idx + 1, op1_mode);
    let operand_two = yank(segments, opcode_idx + 2, op2_mode);
    let target_idx = yank(segments, opcode_idx + 3, Mode::Immediate) as usize;
    (operand_one, operand_two, target_idx)
}

fn op_add(segments: &mut Vec<i32>, opcode_idx: &mut usize, modes: Modes) {
    let (operand_one, operand_two, target_idx) = get_binary_op_chunk(segments, *opcode_idx, modes);
    segments[target_idx] = operand_one + operand_two;
    *opcode_idx += 4;
}

fn op_mul(segments: &mut Vec<i32>, opcode_idx: &mut usize, modes: Modes) {
    let (operand_one, operand_two, target_idx) = get_binary_op_chunk(segments, *opcode_idx, modes);
    segments[target_idx] = operand_one * operand_two;
    *opcode_idx += 4;
}

fn op_read(segments: &mut Vec<i32>, opcode_idx: &mut usize, Modes(mode, _, _): Modes) {
    let target_idx = out_address(segments, *opcode_idx + 1, mode);
    let mut response = String::new();
    print!("Enter parameter for op at {}: ", *opcode_idx);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut response)
        .expect("failed to get param from terminal");
    let val: i32 = response.trim().parse().unwrap();
    segments[target_idx] = val;
    *opcode_idx += 2;
}

fn op_write(segments: &mut Vec<i32>, opcode_idx: &mut usize, Modes(mode, _, _): Modes) {
    println!(
        "Write at index {}: {}",
        *opcode_idx,
        yank(segments, *opcode_idx + 1, mode)
    );
    *opcode_idx += 2;
}

fn op_jump_if_true(
    segments: &mut Vec<i32>,
    opcode_idx: &mut usize,
    Modes(bool_mode, addr_mode, _): Modes,
) {
    match yank(segments, *opcode_idx + 1, bool_mode) {
        0 => *opcode_idx += 3,
        _ => *opcode_idx = yank(segments, *opcode_idx + 2, addr_mode) as usize,
    }
}

fn op_jump_if_false(
    segments: &mut Vec<i32>,
    opcode_idx: &mut usize,
    Modes(bool_mode, addr_mode, _): Modes,
) {
    match yank(segments, *opcode_idx + 1, bool_mode) {
        0 => *opcode_idx = yank(segments, *opcode_idx + 2, addr_mode) as usize,
        _ => *opcode_idx += 3,
    }
}

fn op_less_than(segments: &mut Vec<i32>, opcode_idx: &mut usize, modes: Modes) {
    let (operand_one, operand_two, target_idx) = get_binary_op_chunk(segments, *opcode_idx, modes);
    segments[target_idx] = if operand_one < operand_two { 1 } else { 0 };
    *opcode_idx += 4;
}

fn op_equals(segments: &mut Vec<i32>, opcode_idx: &mut usize, modes: Modes) {
    let (operand_one, operand_two, target_idx) = get_binary_op_chunk(segments, *opcode_idx, modes);
    segments[target_idx] = if operand_one == operand_two { 1 } else { 0 };
    *opcode_idx += 4;
}
