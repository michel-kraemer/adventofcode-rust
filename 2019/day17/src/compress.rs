use crate::instruction::{Func, Instruction};

pub struct Compressed {
    pub main: String,
    pub a: String,
    pub b: String,
    pub c: String,
}

fn instructions_to_string(instructions: &[Instruction]) -> String {
    instructions
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn find_pattern(instructions: &[Instruction], max_len: usize) -> Option<&[Instruction]> {
    let start = instructions
        .iter()
        .position(|i| matches!(i, Instruction::Move(_, _)))?;
    let end = (start + max_len).min(instructions.len());
    let len = end - start;
    let pattern = &instructions[start..end];

    if pattern.iter().any(|i| matches!(i, Instruction::Call(_))) {
        return None;
    }

    let mut i = end;
    while i + len < instructions.len() {
        if &instructions[i..i + len] == pattern {
            return Some(pattern);
        }
        i += 1;
    }

    None
}

fn replace_pattern(
    instructions: &[Instruction],
    pattern: &[Instruction],
    func: Func,
) -> Vec<Instruction> {
    let mut result = Vec::new();
    let mut i = 0;
    while i < instructions.len() {
        if i + pattern.len() <= instructions.len() && &instructions[i..i + pattern.len()] == pattern
        {
            result.push(Instruction::Call(func));
            i += pattern.len();
        } else {
            result.push(instructions[i]);
            i += 1;
        }
    }
    result
}

fn compress_pattern(
    instructions: &[Instruction],
    max_len: usize,
    func: Func,
) -> Option<(Vec<Instruction>, String)> {
    if let Some(a) = find_pattern(instructions, max_len) {
        let a_str = instructions_to_string(a);
        if a_str.len() <= 20 {
            return Some((replace_pattern(instructions, a, func), a_str));
        }
    }
    None
}

/// Very naive implementation, but the list of instructions in this puzzle
/// is very short, so I couldn't care less
pub fn compress(instructions: &[Instruction]) -> Option<Compressed> {
    let mut max_len_a = instructions.len() / 2;
    while max_len_a > 0 {
        if let Some((instructions_a, a)) = compress_pattern(instructions, max_len_a, Func::A) {
            let mut max_len_b = instructions_a.len() / 2;
            while max_len_b > 0 {
                if let Some((instructions_b, b)) =
                    compress_pattern(&instructions_a, max_len_b, Func::B)
                {
                    let mut max_len_c = instructions_b.len() / 2;
                    while max_len_c > 0 {
                        if let Some((instructions_c, c)) =
                            compress_pattern(&instructions_b, max_len_c, Func::C)
                            && instructions_c
                                .iter()
                                .all(|i| matches!(i, Instruction::Call(_)))
                        {
                            return Some(Compressed {
                                main: instructions_to_string(&instructions_c),
                                a,
                                b,
                                c,
                            });
                        }
                        max_len_c -= 1;
                    }
                }
                max_len_b -= 1;
            }
        }
        max_len_a -= 1;
    }
    None
}
