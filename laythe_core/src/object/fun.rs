use laythe_env::managed::{DebugHeap, DebugWrap, Gc, GcStr, Manage, Trace};
use std::{fmt, io::Write, mem};

use crate::{
  chunk::{AlignedByteCode, Chunk, ChunkBuilder},
  module::Module,
  signature::Arity,
  value::Value,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FunKind {
  Fun,
  Method,
  StaticMethod,
  Initializer,
  Script,
}

impl fmt::Display for FunKind {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let string = match self {
      FunKind::Fun => "function",
      FunKind::Method => "method",
      FunKind::StaticMethod => "static method",
      FunKind::Initializer => "class initializer",
      FunKind::Script => "script",
    };

    f.write_str(string)
  }
}

#[derive(Clone)]
pub struct TryBlock {
  /// Start of the try block
  start: u16,

  /// End of the try block
  end: u16,
}

impl TryBlock {
  pub fn new(start: u16, end: u16) -> Self {
    TryBlock { start, end }
  }
}

/// A mutable builder for an immutable function
pub struct FunBuilder {
  /// Name if not top-level script
  name: GcStr,

  /// Arity of this function
  arity: Arity,

  /// Number of upvalues
  upvalue_count: usize,

  /// The module this function belongs to
  module: Gc<Module>,

  /// Catch block present in this function
  try_blocks: Vec<TryBlock>,

  /// Code for the function body
  chunk: ChunkBuilder,
}

impl FunBuilder {
  pub fn new(name: GcStr, module: Gc<Module>) -> Self {
    Self {
      arity: Arity::default(),
      upvalue_count: 0,
      chunk: ChunkBuilder::default(),
      module,
      name,
      try_blocks: Vec::new(),
    }
  }

  /// Increment the upvalue count of this function
  /// builder
  #[inline]
  pub fn inc_upvalue(&mut self) {
    self.upvalue_count += 1;
  }

  /// Set the arity of this function
  pub fn set_arity(&mut self, arity: Arity) {
    self.arity = arity;
  }

  /// Retrieve the current count of upvalues
  #[inline]
  pub fn upvalue_count(&self) -> usize {
    self.upvalue_count
  }

  /// Retrieve a reference to the underlying chunk builder
  #[inline]
  pub fn chunk(&self) -> &ChunkBuilder {
    &self.chunk
  }

  /// Write an aligned byte code to this function
  #[inline]
  pub fn write_instruction(&mut self, op_code: AlignedByteCode, line: u32) {
    self.chunk.write_instruction(op_code, line)
  }

  /// Patch an instruction on this function
  #[inline]
  pub fn patch_instruction(&mut self, index: usize, byte: u8) {
    self.chunk.patch_instruction(index, byte);
  }

  /// Add a constant to this function
  #[inline]
  pub fn add_constant(&mut self, constant: Value) -> usize {
    self.chunk.add_constant(constant)
  }

  /// Add a try block to this function
  pub fn add_try(&mut self, try_block: TryBlock) {
    self.try_blocks.push(try_block)
  }

  /// Build a final immutable Fun from this builder
  pub fn build(self) -> Fun {
    Fun {
      name: self.name,
      arity: self.arity,
      upvalue_count: self.upvalue_count,
      module_id: self.module.id(),
      module: self.module,
      try_blocks: self.try_blocks.into_boxed_slice(),
      chunk: self.chunk.build(),
    }
  }
}

impl Trace for FunBuilder {
  fn trace(&self) {
    self.name.trace();
    self.chunk.trace();
    self.module.trace();
  }

  fn trace_debug(&self, log: &mut dyn Write) {
    self.name.trace_debug(log);
    self.chunk.trace_debug(log);
    self.module.trace_debug(log);
  }
}

#[derive(Clone)]
pub struct Fun {
  /// Name of this function
  name: GcStr,

  /// Arity of this function
  arity: Arity,

  /// Number of upvalues
  upvalue_count: usize,

  /// What is the idea of the associated module
  module_id: usize,

  /// The module this function belongs to
  module: Gc<Module>,

  /// Catch block present in this function
  try_blocks: Box<[TryBlock]>,

  /// Code for the function body
  chunk: Chunk,
}

impl Fun {
  #[cfg(test)]
  pub fn test(name: GcStr, module: Gc<Module>) -> Fun {
    let mut builder = FunBuilder::new(name, module);
    builder.set_arity(Arity::default());

    builder.build()
  }

  /// Name of this function
  #[inline]
  pub fn name(&self) -> GcStr {
    self.name
  }

  /// Arity of this function
  #[inline]
  pub fn arity(&self) -> &Arity {
    &self.arity
  }

  /// Code for the function body
  #[inline]
  pub fn chunk(&self) -> &Chunk {
    &self.chunk
  }

  /// The module id this function belongs to
  #[inline]
  pub fn module_id(&self) -> usize {
    self.module_id
  }

  /// The module this function belongs to
  #[inline]
  pub fn module(&self) -> Gc<Module> {
    self.module
  }

  /// Number of upvalues
  #[inline]
  pub fn upvalue_count(&self) -> usize {
    self.upvalue_count
  }

  pub fn has_catch_jump(&self, ip: u16) -> Option<u16> {
    let mut min_range = std::u16::MAX;
    let mut jump = None;

    for try_block in self.try_blocks.iter() {
      if ip >= try_block.start && ip < try_block.end {
        let len = try_block.end - try_block.start;

        if len < min_range {
          min_range = len;
          jump = Some(try_block.end);
        }
      }
    }

    jump
  }
}

impl fmt::Display for Fun {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "<fn {}>", self.name)
  }
}

impl fmt::Debug for Fun {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.fmt_heap(f, 1)
  }
}

impl Trace for Fun {
  fn trace(&self) {
    self.name.trace();
    self.chunk.trace();
    self.module.trace();
  }

  fn trace_debug(&self, log: &mut dyn Write) {
    self.name.trace_debug(log);
    self.chunk.trace_debug(log);
    self.module.trace_debug(log);
  }
}

impl DebugHeap for Fun {
  fn fmt_heap(&self, f: &mut fmt::Formatter, depth: usize) -> fmt::Result {
    let depth = depth.saturating_sub(1);

    f.debug_struct("Fun")
      .field("name", &DebugWrap(&self.name, depth))
      .field("arity", &self.arity)
      .field("upvalue_count", &self.upvalue_count)
      .field("Module", &DebugWrap(&self.module, depth))
      .field("chunk", &self.chunk)
      .finish()
  }
}

impl Manage for Fun {
  fn size(&self) -> usize {
    mem::size_of::<Self>() + self.chunk.size() + mem::size_of::<TryBlock>() * self.try_blocks.len()
  }

  fn as_debug(&self) -> &dyn DebugHeap {
    self
  }
}