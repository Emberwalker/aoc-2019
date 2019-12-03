use std::collections::HashSet;
use std::fmt;
use std::io;
use std::io::Write;
use std::ops;
use std::env;
use std::iter::FromIterator;

fn main() {
    // Call with --signal-delay to calculate part 2 answer
    let use_signal_delay = env::args()
        .nth(1)
        .map_or(false, |arg| "--signal-delay".eq(&arg));

    let l1 = get_line(1);
    let l2 = get_line(2);

    let (l1_coords, l1_path) = build_coord_set(&l1);
    let (l2_coords, l2_path) = build_coord_set(&l2);

    let intersection: Vec<&Coord> = l1_coords.intersection(&l2_coords).collect();
    println!("Intersection points: {:?}", intersection);

    if use_signal_delay {
        let min_steps: i32 = intersection.iter().fold(i32::max_value(), |min, coord| {
            let l1_steps = l1_path.iter().position(|c| *c == **coord).unwrap() as i32;
            let l2_steps = l2_path.iter().position(|c| *c == **coord).unwrap() as i32;
            min.min(l1_steps + l2_steps + 2) // +2 accounts for the hidden first steps for each line
        });
        println!("Minimum steps: {}", min_steps);
    } else {
        let min_dist: i32 = intersection.iter().fold(i32::max_value(), |min, coord| {
            let dist = coord.manhatten();
            min.min(dist)
        });
        println!("Minimum Manhatten: {}", min_dist);
    }
}

fn get_line(id: i32) -> Vec<Instruction> {
    let mut raw = String::new();
    print!("Enter line {} string: ", id);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut raw)
        .expect("failed to get line from terminal");

    raw.split(',')
        .map(|s| Instruction::from_str(s.trim()))
        .collect()
}

fn build_coord_set(instrs: &[Instruction]) -> (HashSet<Coord>, Vec<Coord>) {
    let mut coords: Vec<Coord> = Vec::new();
    let mut pos = Coord(0, 0);

    for instr in instrs {
        println!("POS {:?}, INST {:?}", pos, instr);
        let mutation = match instr.direction {
            'U' => Coord(0, 1),
            'R' => Coord(1, 0),
            'D' => Coord(0, -1),
            'L' => Coord(-1, 0),
            dir => panic!("Unknown direction: {}", dir),
        };
        for _ in 0..instr.distance {
            pos += mutation;
            coords.push(pos.clone());
        }
    }

    (HashSet::from_iter(coords.iter().copied()), coords)
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Coord(i32, i32);

impl ops::AddAssign for Coord {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Coord {
    fn manhatten(&self) -> i32 {
        self.0.abs() + self.1.abs()
    }
}

#[derive(PartialEq, Eq)]
struct Instruction {
    direction: char,
    distance: i32,
}

impl fmt::Debug for Instruction {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(&format!("<{}{}>", self.direction, self.distance))
    }
}

impl Instruction {
    fn from_str(s: &str) -> Instruction {
        Instruction {
            direction: s.chars().nth(0).unwrap(),
            distance: s.chars().skip(1).collect::<String>().parse().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn parse_instrs() {
        assert_eq!(
            Instruction::from_str("U21"),
            Instruction {
                direction: 'U',
                distance: 21
            }
        );
    }

    #[test]
    fn simple_path() {
        let instrs: Vec<Instruction> = ["U2", "L2", "D1", "R1"]
            .iter()
            .map(|s| Instruction::from_str(s))
            .collect();
        let expected_path = [
            Coord(0, 1),
            Coord(0, 2),
            Coord(-1, 2),
            Coord(-2, 2),
            Coord(-2, 1),
            Coord(-1, 1),
        ].to_vec();
        let expected = HashSet::from_iter(
            expected_path
            .iter()
            .copied()
        );
        assert_eq!(build_coord_set(&instrs), (expected, expected_path));
    }
}
