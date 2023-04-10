use crate::parse::{Ast, Concat, Literal};

pub fn compile(parsed: &Ast) -> Program {
    let mut program = Program {
        instructions: vec![],
    };
    let mut compiled = compile_ast(parsed);

    program.instructions.append(&mut compiled);

    program.instructions.push(Instruction::Match);

    program
}

fn compile_ast(ast: &Ast) -> Vec<Instruction> {
    let mut instructions = vec![];

    match ast {
        Ast::Literal(literal) => {
            let instruction = compile_literal(literal);
            instructions.push(instruction);
        }
        Ast::Concat(concat) => {
            let mut next_instructions = compile_concat(concat);
            instructions.append(&mut next_instructions);
        }
    }

    instructions
}

fn compile_concat(concat: &Concat) -> Vec<Instruction> {
    let mut instructions = vec![];

    for ast in &concat.asts {
        instructions.append(&mut compile_ast(&ast));
    }

    instructions
}

fn compile_literal(literal: &Literal) -> Instruction {
    Instruction::Char(literal.c)
}

#[derive(Debug, PartialEq, Eq)]
pub struct Program {
    pub instructions: Vec<Instruction>,
}

pub type InstructionPointer = usize;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Instruction {
    Match,
    Char(char),
    Jump(InstructionPointer),
    Split(InstructionPointer, InstructionPointer),
}

#[cfg(test)]
mod tests {
    use crate::parse::parse;

    use super::*;

    #[test]
    fn parse_literals() {
        const PATTERN: &str = "abcde";

        let program = compile(&parse(PATTERN));

        assert_eq!(
            program,
            Program {
                instructions: vec![
                    Instruction::Char('a'),
                    Instruction::Char('b'),
                    Instruction::Char('c'),
                    Instruction::Char('d'),
                    Instruction::Char('e'),
                    Instruction::Match
                ]
            }
        )
    }
}
