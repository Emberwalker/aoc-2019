use std::mem;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Instruction {
    pub opcode: Opcode,
    pub modes: Modes,
}

impl From<i32> for Instruction {
    fn from(instr: i32) -> Self {
        let digits = to_digits(instr.abs() as u32);
        let modes = Modes(Mode::from_digit(digits[2]), Mode::from_digit(digits[1]), Mode::from_digit(digits[1]));
        let opcode = Opcode::from_int((digits[3] * 10) + digits[4]);
        Instruction { opcode, modes }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Opcode {
    Add,
    Mul,
    Read,
    Write,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    End,
}

impl Opcode {
    pub fn from_int(int: u32) -> Opcode {
        match int {
            1 => Opcode::Add,
            2 => Opcode::Mul,
            3 => Opcode::Read,
            4 => Opcode::Write,
            5 => Opcode::JumpIfTrue,
            6 => Opcode::JumpIfFalse,
            7 => Opcode::LessThan,
            8 => Opcode::Equals,
            99 => Opcode::End,
            x => panic!("Invalid opcode: {}", x),
        }
    }
}

impl PartialEq for Opcode {
    fn eq(&self, other: &Self) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }
}

impl Eq for Opcode {}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Modes(pub Mode, pub Mode, pub Mode);

#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Position,
    Immediate,
}

impl Mode {
    pub fn from_digit(digit: u32) -> Mode {
        match digit {
            0 => Mode::Position,
            1 => Mode::Immediate,
            x => panic!("Invalid mode: {}", x),
        }
    }
}

impl PartialEq for Mode {
    fn eq(&self, other: &Self) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }
}

impl Eq for Mode {}

fn to_digits(i: u32) -> Vec<u32> {
    // Pad to 5 digits
    format!("{:05}", i)
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

#[cfg(test)]
mod test {

    use crate::types::*;

    #[test]
    fn test_parse_instr_simple_add() {
        assert_eq!(
            Instruction::from(1),
            Instruction {
                opcode: Opcode::Add,
                modes: Modes(Mode::Position, Mode::Position, Mode::Position),
            }
        )
    }

    #[test]
    fn test_parse_instr_immediate_mul() {
        assert_eq!(
            Instruction::from(1102),
            Instruction {
                opcode: Opcode::Mul,
                modes: Modes(Mode::Position, Mode::Immediate, Mode::Immediate),
            }
        )
    }

}
