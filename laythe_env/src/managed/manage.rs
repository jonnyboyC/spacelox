use std::{fmt, io::Write};

/// A wrapper struct to print debug information to
/// a provided depth. This is to handled cycles in
/// managed Gc pointers
pub struct DebugWrap<'a, T>(pub &'a T, pub usize);

impl<'a, T: DebugHeap> fmt::Debug for DebugWrap<'a, T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.0.fmt_heap(f, self.1)
  }
}

/// A wrapper struct to print debug information to
/// a provided depth. This is to handled cycles in
/// managed Gc pointers
pub struct DebugWrapDyn<'a>(pub &'a dyn DebugHeap, pub usize);

impl<'a> fmt::Debug for DebugWrapDyn<'a> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.0.fmt_heap(f, self.1)
  }
}

/// A utility to print debug information to a fixed depth in the Laythe heap
pub trait DebugHeap {
  /// A debugging string for this managed object. Typically just wrapping
  /// wrapping fmt::Debug so we can have dyn Manage
  fn fmt_heap(&self, f: &mut fmt::Formatter, depth: usize) -> fmt::Result;
}

/// An entity that is traceable by the garbage collector
pub trait Trace {
  /// Mark all objects that are reachable from this object
  fn trace(&self) {}

  /// Mark all objects that are reachable printing debugging information
  /// for each object
  fn trace_debug(&self, _log: &mut dyn Write) {}
}

/// An entity that can provide tracing roots to the garbage collector
pub trait TraceRoot {
  /// Mark all objects that are reachable from this object
  fn trace(&self);

  /// Mark all objects that are reachable printing debugging information
  /// for each object
  fn trace_debug(&self, log: &mut dyn Write);

  /// Are we in a context were we can collect garbage.
  fn can_collect(&self) -> bool;
}

/// An entity that can be managed and collected by the garbage collector.
/// This trait provided debugging capabilities and statistics for the gc.
pub trait Manage: Trace + DebugHeap {
  /// What is the size of this allocation
  fn size(&self) -> usize;

  /// Helper function to get a trait object for Debug Heap
  fn as_debug(&self) -> &dyn DebugHeap;
}
