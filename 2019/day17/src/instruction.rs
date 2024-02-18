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

impl ToString for Instruction {
    fn to_string(&self) -> String {
        match self {
            Instruction::Move(turn, n) => match turn {
                Turn::Left => format!("L,{}", n),
                Turn::Right => format!("R,{}", n),
            },
            Instruction::Call(f) => match f {
                Func::A => "A".to_string(),
                Func::B => "B".to_string(),
                Func::C => "C".to_string(),
            },
        }
    }
}
