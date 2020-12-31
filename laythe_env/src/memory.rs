use crate::managed::{Allocation, Gc, Manage, RootTrace, Trace};
use crate::stdio::Stdio;
use hashbrown::HashMap;
use smol_str::SmolStr;
use std::ptr::NonNull;
use std::{cell::RefCell, io::Write};

#[cfg(feature = "debug_gc")]
use crate::managed::{DebugWrap, DebugWrapDyn};

/// The garbage collector and memory manager for laythe. Currently this is implemented a very crude
/// generation mark and sweep collector. As of now the key areas for improvements are better allocation
/// strategy and better tuning of the interaction between the nursery and regular heap
pub struct Allocator {
  /// Io in the given environment
  #[allow(dead_code)]
  stdio: RefCell<Stdio>,

  /// The nursery heap for new objects initially allocated into this gc
  nursery_heap: Vec<Box<Allocation<dyn Manage>>>,

  /// The regular heap where objects that have survived a gc reside
  heap: Vec<Box<Allocation<dyn Manage>>>,

  /// A collection of temporary roots in the gc
  temp_roots: Vec<Box<dyn Trace>>,

  /// The total byte allocated in both heaps
  bytes_allocated: usize,

  /// The intern string cache
  intern_cache: HashMap<&'static str, Gc<SmolStr>>,

  /// The size in bytes of the gc before the next collection
  next_gc: usize,

  /// The total number of garbage collections that have occured
  gc_count: u128,
}

const GC_HEAP_GROW_FACTOR: usize = 2;

impl<'a> Allocator {
  /// Create a new manged heap for laythe for objects.
  ///
  /// # Examples
  /// ```
  /// use laythe_env::memory::Allocator;
  /// use laythe_env::stdio::Stdio;
  ///
  /// let gc = Allocator::new(Stdio::default());
  /// ```
  pub fn new(stdio: Stdio) -> Self {
    Self {
      stdio: RefCell::new(stdio),
      nursery_heap: Vec::with_capacity(1000),
      heap: vec![],
      bytes_allocated: 0,
      temp_roots: vec![],
      intern_cache: HashMap::new(),
      next_gc: 1024 * 1024,
      gc_count: 0,
    }
  }

  /// Get the number of bytes allocated
  pub fn allocated(&self) -> usize {
    self.bytes_allocated
  }

  /// Create a `Managed<T>` from the provided `data`. This method will allocate space
  /// for `data` and return a pointer to it. In case of a gc the provided `context` is
  /// used to annotate active roots
  ///
  /// # Examples
  /// ```
  /// use laythe_env::memory::{Allocator, NO_GC};
  /// use smol_str::SmolStr;
  ///
  /// let mut gc = Allocator::default();
  /// let string = gc.manage(SmolStr::from("example"), &NO_GC);
  ///
  /// assert_eq!(&*string, "example");
  /// ```
  pub fn manage<T: 'static + Manage, C: RootTrace + ?Sized>(
    &mut self,
    data: T,
    context: &C,
  ) -> Gc<T> {
    self.allocate(data, context)
  }

  /// Create a `Managed<String>` from a str slice. This creates
  /// or returns an interned string and allocates a pointer to the intern
  /// cache. A Managed<String> can be created from `.manage` but will
  /// not intern the string.
  ///
  /// # Examples
  /// ```
  /// use laythe_env::memory::{Allocator, NO_GC};
  ///
  /// let mut gc = Allocator::default();
  /// let str = gc.manage_str("hi!", &NO_GC);
  ///
  /// assert_eq!(&*str, "hi!");
  /// ```
  pub fn manage_str<S: Into<String> + AsRef<str>, C: RootTrace + ?Sized>(
    &mut self,
    src: S,
    context: &C,
  ) -> Gc<SmolStr> {
    let string = SmolStr::from(src);
    if let Some(cached) = self.intern_cache.get(&*string) {
      return *cached;
    }

    let managed = self.allocate(string, context);
    let static_str: &'static str = unsafe { &*(&**managed as *const str) };
    self.intern_cache.insert(&static_str, managed);
    managed
  }

  /// track events that may grow the size of the heap. If
  /// a heap grows beyond the current threshold will trigger a gc
  pub fn grow<T: 'static + Manage, R, F: FnOnce(&mut T) -> R, C: RootTrace + ?Sized>(
    &mut self,
    managed: &mut T,
    context: &C,
    action: F,
  ) -> R {
    let before = managed.size();
    let result = action(managed);
    let after = managed.size();

    // get the size delta before and after the action
    // this would occur because of some resize
    self.bytes_allocated += after - before;

    // collect if need be
    #[cfg(feature = "debug_stress_gc")]
    {
      self.collect_garbage(context);
    }

    if self.bytes_allocated > self.next_gc {
      self.collect_garbage(context);
    }

    result
  }

  /// track events that may shrink the size of the heap.
  pub fn shrink<T: 'static + Manage, R, F: FnOnce(&mut T) -> R>(
    &mut self,
    managed: &mut T,
    action: F,
  ) -> R {
    let before = managed.size();
    let result = action(managed);
    let after = managed.size();

    // get the size delta before and after the action
    // this would occur because of some resize
    self.bytes_allocated += before - after;

    result
  }

  /// Push a new temporary root onto the gc to avoid collection
  pub fn push_root<T: 'static + Manage>(&mut self, managed: T) {
    self.temp_roots.push(Box::new(managed));
  }

  /// Pop a temporary root again allowing gc to occur normally
  pub fn pop_roots(&mut self, count: usize) {
    self.temp_roots.truncate(self.temp_roots.len() - count);
  }

  /// Allocate `data` on the gc's heap. If conditions are met
  /// a garbage collection can be triggered. When triggered will use the
  /// context to determine the active roots.
  fn allocate<T: 'static + Manage, C: RootTrace + ?Sized>(
    &mut self,
    data: T,
    context: &C,
  ) -> Gc<T> {
    // create own store of allocation
    let mut alloc = Box::new(Allocation::new(data));
    let ptr = unsafe { NonNull::new_unchecked(&mut *alloc) };

    let size = alloc.size();

    // push onto heap
    self.bytes_allocated += size;
    self.nursery_heap.push(alloc);

    let managed = Gc::from(ptr);

    #[cfg(feature = "debug_gc")]
    self.debug_allocate(ptr, size);

    #[cfg(feature = "debug_stress_gc")]
    {
      self.push_root(managed);
      self.collect_garbage(context);
      self.pop_roots(1)
    }

    if self.bytes_allocated > self.next_gc {
      self.push_root(managed);
      self.collect_garbage(context);
      self.pop_roots(1)
    }

    managed
  }

  /// Collect garbage present in the heap for unreachable objects. Use the provided context
  /// to mark a set of initial roots into the vm.
  fn collect_garbage<C: RootTrace + ?Sized>(&mut self, context: &C) {
    #[cfg(feature = "debug_gc")]
    let before = self.bytes_allocated;
    self.gc_count += 1;

    #[cfg(feature = "debug_gc")]
    {
      let mut stdio = self.stdio.borrow_mut();
      let stdout = stdio.stdout();
      writeln!(stdout, "-- gc begin {} --", self.gc_count).expect("could not write to stdout");
    }

    if self.trace_root(context) {
      self.temp_roots.iter().for_each(|root| {
        self.trace(&**root);
      });

      self.sweep_string_cache();
      self.bytes_allocated = self.sweep();
      self.next_gc = self.bytes_allocated * GC_HEAP_GROW_FACTOR
    }

    #[cfg(feature = "debug_gc")]
    {
      let mut stdio = self.stdio.borrow_mut();
      let stdout = stdio.stdout();
      let now = self.bytes_allocated;

      writeln!(stdout, "-- gc end --").expect("unable to write to stdout");
      debug_assert!(
        before >= now,
        "Heap was incorrectly calculated before: {} now {}",
        before,
        now
      );

      writeln!(
        stdout,
        "   collected {} bytes (from {} to {}) next at {}",
        before.saturating_sub(now),
        before,
        now,
        self.next_gc
      )
      .expect("unable to write to stdout");
    }
  }

  /// wrapper around a roots trace method to select either normal
  /// or debug trace at compile time.
  fn trace_root<C: RootTrace + ?Sized>(&self, context: &C) -> bool {
    #[cfg(not(feature = "debug_gc"))]
    return context.trace();

    #[cfg(feature = "debug_gc")]
    {
      let mut stdio = self.stdio.borrow_mut();
      let stdout = stdio.stdout();

      return context.trace_debug(stdout);
    }
  }

  /// wrapper around an entities trace method to select either normal
  /// or debug trace at compile time.
  fn trace(&self, entity: &dyn Trace) -> bool {
    #[cfg(not(feature = "debug_gc"))]
    return entity.trace();

    #[cfg(feature = "debug_gc")]
    {
      let mut stdio = self.stdio.borrow_mut();
      let stdout = stdio.stdout();
      return entity.trace_debug(stdout);
    }
  }

  /// Remove unmarked objects from the heap. This calculates the remaining
  /// memory present in the heap
  fn sweep(&mut self) -> usize {
    #[cfg(feature = "debug_stress_gc")]
    return self.sweep_full();

    #[cfg(not(feature = "debug_stress_gc"))]
    if self.gc_count % 10 == 0 {
      self.sweep_full()
    } else {
      self.sweep_nursery()
    }
  }

  /// Remove unmarked objects from the nursery heap. Promoting surviving objects
  /// to the normal heap
  #[cfg(not(feature = "debug_stress_gc"))]
  fn sweep_nursery(&mut self) -> usize {
    self.heap.extend(self.nursery_heap.drain(..).filter(|obj| {
      let retain = (*obj).marked();

      #[cfg(feature = "debug_gc")]
      debug_free(&obj, !retain);

      retain
    }));

    let mut remaining: usize = 0;
    self.heap.iter().for_each(|obj| {
      (*obj).unmark();
      remaining += obj.size();
    });

    remaining
  }

  /// Remove unmarked objects from the both heaps. Promoting surviving objects
  /// to the normal heap
  fn sweep_full(&mut self) -> usize {
    let mut remaining: usize = 0;

    self.heap.extend(self.nursery_heap.drain(..).filter(|obj| {
      let retain = (*obj).marked();

      #[cfg(feature = "debug_gc")]
      debug_free(&obj, !retain);

      retain
    }));

    self.heap.retain(|obj| {
      let retain = (*obj).unmark();

      #[cfg(feature = "debug_gc")]
      debug_free(&obj, !retain);

      if retain {
        remaining += obj.size();
        return true;
      }

      false
    });

    remaining
  }

  /// Remove strings from the cache that no longer have any references
  /// in the heap
  fn sweep_string_cache(&mut self) {
    self.intern_cache.retain(|_, &mut string| {
      #[allow(clippy::let_and_return)]
      let retain = string.obj().marked();

      #[cfg(feature = "debug_gc")]
      debug_string_remove(string, !retain);

      retain
    });
  }

  /// Debug logging for allocating an object.
  #[cfg(feature = "debug_gc")]
  fn debug_allocate<T: 'static + Manage>(&self, ptr: NonNull<Allocation<T>>, size: usize) {
    #[cfg(feature = "debug_gc")]
    {
      let mut stdio = self.stdio.borrow_mut();
      let stdout = stdio.stdout();

      writeln!(
        stdout,
        "{:p} allocated {} bytes for {:?}",
        ptr.as_ptr(),
        size,
        DebugWrap(unsafe { ptr.as_ref() }, 1)
      )
      .expect("unable to write to stdout");
    }
  }
}

/// Debug logging for removing a string from the cache.
#[cfg(feature = "debug_gc")]
fn debug_string_remove(string: Gc<SmolStr>, free: bool) {
  if free {
    println!(
      "{:p} remove string from cache {:?}",
      &**string,
      DebugWrap(&string, 1)
    )
  }
}

/// Debug logging for free an object.
#[cfg(feature = "debug_gc")]
fn debug_free(obj: &Box<Allocation<dyn Manage>>, free: bool) {
  if free {
    println!(
      "{:p} free {} bytes from {:?}",
      &**obj,
      obj.size(),
      DebugWrapDyn((*obj).as_debug(), 0)
    )
  }
}

impl<'a> Default for Allocator {
  fn default() -> Self {
    Allocator::new(Stdio::default())
  }
}
pub struct NoGc();

impl RootTrace for NoGc {
  fn trace(&self) -> bool {
    false
  }

  fn trace_debug(&self, _: &mut dyn Write) -> bool {
    false
  }
}

pub static NO_GC: NoGc = NoGc();

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn dyn_manage() {
    let dyn_trace: Box<dyn RootTrace> = Box::new(NoGc());
    let mut gc = Allocator::default();

    let dyn_manged_str = gc.manage(SmolStr::from("managed"), &*dyn_trace);
    assert_eq!(*dyn_manged_str, SmolStr::from("managed"));
  }
}
