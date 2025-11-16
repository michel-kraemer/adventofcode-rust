use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Turn {
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Func {
    A,
    B,
    C,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Move(Turn, usize),
    Call(Func),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Move(turn, n) => match turn {
                Turn::Left => write!(f, "L,{}", n),
                Turn::Right => write!(f, "R,{}", n),
            },
            Instruction::Call(fun) => match fun {
                Func::A => write!(f, "A"),
                Func::B => write!(f, "B"),
                Func::C => write!(f, "C"),
            },
        }
    }
}
