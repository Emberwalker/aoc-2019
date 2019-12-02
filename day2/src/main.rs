use std::env;
use std::io;
use std::io::Write;

fn main() {
    // Call with --solve-for-value to calculate part 2 answer
    let solve_for_value = env::args()
        .nth(1)
        .map_or(false, |arg| "--solve-for-value".eq(&arg));

    let mut raw_program = String::new();
    print!("Enter Intcode string: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut raw_program)
        .expect("failed to get Intcode from terminal");

    let mut segments: Vec<i32> = Vec::new();
    for str_segment in raw_program.split(',') {
        let trimmed_segment = str_segment.trim();
        let segment: i32 = trimmed_segment.parse().unwrap_or_else(|_| panic!("failed to parse segment {}", trimmed_segment));
        segments.push(segment);
    }

    if solve_for_value {
        for noun in 0..100 {
            println!("Trying with noun = {}...", noun);
            for verb in 0..100 {
                let mut iteration_segments = segments.clone();
                *iteration_segments.get_mut(1).unwrap() = noun;
                *iteration_segments.get_mut(2).unwrap() = verb;
                execute(&mut iteration_segments);

                if yank(&iteration_segments, 0) == 19_690_720 {
                    println!("SOLUTION FOUND: Verb {}, Noun {}, Derived {}", verb, noun, 100 * noun + verb);
                    return;
                }
            }
        }
    } else {
        execute(&mut segments);
        println!("Program END: {:?}", segments);
    }
}

fn execute(segments: &mut Vec<i32>) {
    let mut idx: usize = 0;
    loop {
        match segments.get(idx).unwrap() {
            1 => op_add(segments, idx),
            2 => op_mul(segments, idx),
            99 => break,
            v => panic!("invalid opcode {} at position {}", v, idx)
        }
        idx += 4;
    }
}

fn yank(segments: &[i32], idx: usize) -> i32 {
    *segments.get(idx).unwrap_or_else(|| panic!("Out of bounds: {}", idx))
}

fn yank_pointer(segments: &[i32], idx: usize) -> i32 {
    yank(segments, yank(segments, idx) as usize)
}

fn get_chunk(segments: &[i32], opcode_idx: usize) -> (i32, i32, usize) {
    let operand_one = yank_pointer(segments, opcode_idx + 1);
    let operand_two = yank_pointer(segments, opcode_idx + 2);
    let target_idx = yank(segments, opcode_idx + 3) as usize;
    (operand_one, operand_two, target_idx)
}

fn op_add(segments: &mut Vec<i32>, opcode_idx: usize) {
    let (operand_one, operand_two, target_idx) = get_chunk(segments, opcode_idx);
    *segments.get_mut(target_idx).unwrap() = operand_one + operand_two;
}

fn op_mul(segments: &mut Vec<i32>, opcode_idx: usize) {
    let (operand_one, operand_two, target_idx) = get_chunk(segments, opcode_idx);
    *segments.get_mut(target_idx).unwrap() = operand_one * operand_two;
}
