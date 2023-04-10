use std::mem;

use crate::compile::{Instruction, InstructionPointer, Program};

/// A (partial) implementation of the PikeVM described in <https://swtch.com/~rsc/regexp/regexp2.html>
/// based on the implementation in regex crate (src/pikevm.rs), heavily simplified.
pub fn execute<'t>(program: &Program, text: &'t str, start: usize) -> bool {
    let program_length = program.instructions.len();
    let mut current_list = Vec::with_capacity(program_length);
    let mut next_list = Vec::with_capacity(program_length);

    let mut at = next_at(text, start);

    loop {
        // Add a thread starting at instruction 0
        // e.g. "start matching the pattern from the current position in the text"
        // This is done for each iteration, implicitly adding a leading .*? to all patterns matched on
        current_list.push(Thread {
            instruction_pointer: 0,
        });

        for thread in &current_list {
            let instruction_pointer = thread.instruction_pointer;

            match program.instructions[instruction_pointer] {
                // We reached a match, meaning that the text matched the pattern
                Instruction::Match => return true,
                // Instruction require that the current character matches the literal character in the instruction
                // If this is true, the thread can continue on.
                // If not the thread dies (we do not add the successor, which implicitly kills it)
                Instruction::Char(c) => {
                    if let Some(at_char) = at.c {
                        if c == at_char {
                            next_list.push(Thread {
                                instruction_pointer: instruction_pointer + 1,
                            });
                        }
                    }
                }
                Instruction::Jump(_) => todo!(),
                Instruction::Split(_, _) => todo!(),
            }
        }

        // End condition: If the current at is past the length of the string, there is no way we can match the pattern
        // and we can safely return false for the match.
        if at.pos() >= text.len() {
            return false;
        }

        at = next_at(text, at.next_pos());
        mem::swap(&mut current_list, &mut next_list);
        next_list.clear();
    }
}

#[derive(Debug)]
struct Thread {
    instruction_pointer: InstructionPointer,
}

/// Represents a location in the input.
#[derive(Clone, Copy, Debug)]
pub struct At {
    pos: usize,
    c: Option<char>,
    len: usize,
}

impl At {
    /// Returns the byte offset of this position.
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Returns the byte offset of the next position in the input.
    pub fn next_pos(&self) -> usize {
        self.pos + self.len
    }
}

fn next_at(text: &str, index: usize) -> At {
    if index >= text.len() {
        At {
            pos: text.len(),
            c: None,
            len: 0,
        }
    } else {
        let c = text.chars().skip(index).next();
        At {
            pos: index,
            c,
            len: c.map_or(1, |c| c.len_utf8()),
        }
    }
}
