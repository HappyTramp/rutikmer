use rand::{
    distributions::{Distribution, Standard},
    Rng
};

#[derive(PartialEq)]
enum MoveDirection {
    Front,
    Back,
    Down,
    Up,
    Right,
    Left,
}

enum MoveModifier {
    None,
    Twice,
    Prime,
}

pub struct Move {
    direction: MoveDirection,
    modifier: MoveModifier,
}


// https://stackoverflow.com/questions/48490049
impl Distribution<MoveDirection> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MoveDirection {
        use MoveDirection::*;

        match rng.gen_range(0, 6) {
            0  => Front,
            1  => Back,
            2  => Down,
            3  => Up,
            4  => Right,
            _  => Left,
        }
    }
}

impl Distribution<MoveModifier> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MoveModifier {
        use MoveModifier::*;

        match rng.gen_range(0, 3) {
            0  => None,
            1  => Twice,
            _  => Prime,
        }
    }
}

impl Move {
    pub fn sequence(n: usize) -> Vec<Move> {
        let mut sequence: Vec<Move> = Vec::with_capacity(n);

        while sequence.len() != n {
            let direction = rand::random::<MoveDirection>();
            let modifier = rand::random::<MoveModifier>();

            if let Some(l) = sequence.last() {
                if l.direction == direction {
                    continue;
                }
            }
            sequence.push(Move { direction, modifier });
        }
        sequence
    }

    pub fn string_sequence(n: usize) -> String {
        let seq = Move::sequence(n);
        seq.iter().map(|m| m.to_string() + " ").collect::<Vec<String>>().join(" ")
    }
}

use std::fmt;

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use MoveDirection::*;
        use MoveModifier::*;

        let letter = match self.direction {
            Front  => "F",
            Back   => "B",
            Down   => "D",
            Up     => "U",
            Right  => "R",
            Left   => "L",
        };
        let modifier = match self.modifier {
            None  => "",
            Twice => "2",
            Prime => "'",
        };
        write!(f, "{}{}", letter, modifier)
    }
}
