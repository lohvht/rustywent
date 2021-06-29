use super::value::{Value, ValuePool};
use std::fmt::{Display, Error, Formatter};

#[derive(Copy, Clone)]
pub enum Instruction {
    Return,
    Constant(u16),
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            Instruction::Return => write!(f, "OP_RETURN"),
            Instruction::Constant(idx) => write!(f, "OP_CONSTANT\t{}", idx),
        }
    }
}

pub struct Chunk {
    instructions: Vec<Instruction>,
    constants: ValuePool,
    linecoltab: Vec<(u16, u16)>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            instructions: Vec::new(),
            constants: ValuePool::new(),
            linecoltab: Vec::new(),
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction, linecol: (u16, u16)) {
        self.linecoltab.push(linecol);
        self.instructions.push(instruction)
    }

    pub fn add_constant_instruction(&mut self, v: Value, linecol: (u16, u16)) {
        let idx = self.constants.add(v);
        self.add_instruction(Instruction::Constant(idx), linecol)
    }

    pub fn print_disassembled(&self, name: &str) {
        println!("== {} ==", name);
        println!("{}", self)
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let op_code_display = self
            .instructions
            .iter()
            .enumerate()
            .map(|(i, code)| {
                let (lineno, colno) = self.linecoltab.get(i).unwrap_or_else(|| {
                    panic!("unable to get line/col info at instruction idx {}", i)
                });
                let prefix = format!("{:04}\t{}:{}\t{}", i, lineno, colno, code.to_string());
                match *code {
                    Instruction::Constant(idx) => {
                        format!("{}\t{:?}", prefix, self.constants.get(idx))
                    }
                    _ => prefix,
                }
            })
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{}", op_code_display)
    }
}

#[cfg(test)]
mod tests;
