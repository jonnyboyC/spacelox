use crate::chunk::{Chunk, OpCode};

/// Write a chunk to console
pub fn disassemble_chunk(code_chunk: &Chunk, name: &str) -> () {
  println!("== {0} ==", name);

  for offset in 0..code_chunk.instructions.len() {
    disassemble_instruction(code_chunk, offset)
  }
}

/// Write an instruction to console
pub fn disassemble_instruction(chunk: &Chunk, offset: usize) {
  print!("{:0>4} ", offset);

  if offset > 0 && chunk.get_line(offset) == chunk.get_line(offset - 1) {
    print!("   | ")
  } else {
    print!("{:>4} ", chunk.get_line(offset))
  }

  let instruction = &chunk.instructions[offset];
  match instruction {
    OpCode::Return => simple_instruction("Return"),
    OpCode::Negate => simple_instruction("Negate"),
    OpCode::Add => simple_instruction("Add"),
    OpCode::Subtract => simple_instruction("Subtract"),
    OpCode::Multiply => simple_instruction("Multiply"),
    OpCode::Divide => simple_instruction("Divide"),
    OpCode::Constant(constant) => constant_instruction("Constant", chunk, constant),
  }
}

/// print a constant
fn constant_instruction(name: &str, chunk: &Chunk, constant: &usize) {
  print!("{} {:4} ", name, constant);
  print_value(chunk.constants.values[*constant]);
  println!();
}

/// print a simple instruction
fn simple_instruction(name: &str) {
  println!("{}", name);
}

/// print a value
fn print_value(value: f64) {
  print!("{}", value);
}