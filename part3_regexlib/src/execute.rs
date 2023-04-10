use crate::compile::{Instruction, InstructionPointer, Program};

/// A (partial) implementation of the PikeVM described in <https://swtch.com/~rsc/regexp/regexp2.html>
/// based on the implementation in regex crate (src/pikevm.rs), heavily simplified.
/// (We do not do any caching, we potentially do redundant execution of instructions etc.)
pub fn execute(program: &Program, text: &str, start: usize) -> bool {
    let program_length = program.instructions.len();
    let mut current_list = Vec::with_capacity(program_length);
    let mut next_list = Vec::with_capacity(program_length);

    // Iterator over the characters in text. Skip until we reach start.
    let mut chars = text.chars().skip(start);

    loop {
        // Add a thread starting at instruction 0
        // e.g. "start matching the pattern from the current position in the text"
        // This is done for each iteration, implicitly adding a leading .*? to all patterns matched on
        current_list.push(Thread::new(0));

        // Advance at to next character in input text
        let at = chars.next();

        // Execute all threads in current_list for the current character, at.
        // Threads in current_list may either yield a match, continue (yield more instructions)
        // or die (yield no more instructions).
        for thread in &current_list {
            let instruction_pointer = thread.instruction_pointer;
            match program.instructions[instruction_pointer] {
                // We reached a match instruction, meaning that the text matched the pattern
                Instruction::Match => return true,
                // Instruction require that the current character matches the literal character in the instruction
                // If this is true, the thread can continue on.
                // If not the thread dies (we do not add the successor, which implicitly kills it)
                Instruction::Char(c) => {
                    if let Some(at_char) = at {
                        if c == at_char {
                            next_list.push(Thread::new(instruction_pointer + 1));
                        }
                    }
                }
                // TODO: Implement Jump instructions. See <https://swtch.com/~rsc/regexp/regexp2.html>
                Instruction::Jump(_) => todo!(),
                // TODO: Implement Split instructions. See <https://swtch.com/~rsc/regexp/regexp2.html>
                Instruction::Split(_, _) => todo!(),
            }
        }

        // End condition: If there are no more characters in the input text, the pattern can never match.
        if at.is_none() {
            return false;
        }

        // Cleanup: Swap the two thread lists (current and next), and clear the next_list for the new iteration
        std::mem::swap(&mut current_list, &mut next_list);
        next_list.clear();
    }
}

/// Thread state
struct Thread {
    /// Pointer (index into instructions vec of program) to current instruction of Thread
    instruction_pointer: InstructionPointer,
}

impl Thread {
    fn new(instruction_pointer: InstructionPointer) -> Self {
        Self {
            instruction_pointer,
        }
    }
}
