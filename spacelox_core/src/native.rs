use crate::value::Value;
use std::fmt;
use std::ptr;
use std::rc::Rc;
use std::time::SystemTime;

pub enum NativeResult {
  /// The result of the native function call was a success with this value
  Success(Value),

  /// The result of the native function call was an error with this runtime
  /// message
  RuntimeError(String),
}

pub trait NativeFun {
  /// Meta data to this native function
  fn meta(&self) -> &NativeMeta;

  /// Call the native functions
  fn call(&self, values: &[Value]) -> NativeResult;

  // /// Check if this native function is equal to another
  fn eq(&self, rhs: &dyn NativeFun) -> bool;
}

impl PartialEq<dyn NativeFun> for dyn NativeFun {
  fn eq(&self, rhs: &dyn NativeFun) -> bool {
    self.eq(rhs)
  }
}

impl fmt::Debug for dyn NativeFun {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let meta = self.meta();
    f.debug_struct("NativeFun")
      .field("name", &meta.name)
      .field("arity", &meta.arity)
      .finish()
  }
}

impl fmt::Display for dyn NativeFun {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let meta = self.meta();
    write!(f, "<native {}>", meta.name)
  }
}

#[derive(Clone, Debug)]
pub struct NativeMeta {
  pub name: String,
  pub arity: u8,
}

pub fn create_natives() -> Vec<Rc<dyn NativeFun>> {
  let mut natives: Vec<Rc<dyn NativeFun>> = Vec::new();

  natives.push(Rc::new(NativeClock::new()));
  natives.push(Rc::new(NativeAssert::new()));
  natives.push(Rc::new(NativeAssertEq::new()));
  natives.push(Rc::new(NativeAssertNe::new()));

  natives
}

fn native_eq(lhs: &dyn NativeFun, rhs: &dyn NativeFun) -> bool {
  ptr::eq(lhs.meta(), rhs.meta())
}
#[derive(Clone, Debug)]
struct NativeClock {
  meta: Box<NativeMeta>,
  start: SystemTime,
}

impl NativeClock {
  pub fn new() -> Self {
    Self {
      meta: Box::new(NativeMeta {
        name: "clock".to_string(),
        arity: 0,
      }),
      start: SystemTime::now(),
    }
  }
}

impl NativeFun for NativeClock {
  fn meta(&self) -> &NativeMeta {
    &self.meta
  }

  fn eq(&self, rhs: &dyn NativeFun) -> bool {
    native_eq(self, rhs)
  }

  fn call(&self, _: &[Value]) -> NativeResult {
    match self.start.elapsed() {
      Ok(elapsed) => NativeResult::Success(Value::Number((elapsed.as_micros() as f64) / 1000000.0)),
      Err(e) => NativeResult::RuntimeError(format!("clock failed {}", e)),
    }
  }
}

#[derive(Clone, Debug)]
struct NativeAssert {
  meta: Box<NativeMeta>,
  start: SystemTime,
}

impl NativeAssert {
  pub fn new() -> Self {
    Self {
      meta: Box::new(NativeMeta {
        name: "assert".to_string(),
        arity: 1,
      }),
      start: SystemTime::now(),
    }
  }
}

impl NativeFun for NativeAssert {
  fn meta(&self) -> &NativeMeta {
    &self.meta
  }

  fn eq(&self, rhs: &dyn NativeFun) -> bool {
    native_eq(self, rhs)
  }

  fn call(&self, args: &[Value]) -> NativeResult {
    match args[0] {
      Value::Bool(b) => {
        if b {
          return NativeResult::Success(Value::Nil);
        }
        NativeResult::RuntimeError(format!("assert expected true received false"))
      }
      _ => NativeResult::RuntimeError(format!("assert expected a boolean value")),
    }
  }
}

#[derive(Clone, Debug)]
struct NativeAssertEq {
  meta: Box<NativeMeta>,
  start: SystemTime,
}

impl NativeAssertEq {
  pub fn new() -> Self {
    Self {
      meta: Box::new(NativeMeta {
        name: "assertEq".to_string(),
        arity: 2,
      }),
      start: SystemTime::now(),
    }
  }
}

impl NativeFun for NativeAssertEq {
  fn meta(&self) -> &NativeMeta {
    &self.meta
  }

  fn eq(&self, rhs: &dyn NativeFun) -> bool {
    native_eq(self, rhs)
  }

  fn call(&self, args: &[Value]) -> NativeResult {
    if args[0] == args[1] {
      return NativeResult::Success(Value::Nil);
    }

    NativeResult::RuntimeError(format!("{:?} and {:?} where not equal", args[0], args[1]))
  }
}

#[derive(Clone, Debug)]
struct NativeAssertNe {
  meta: Box<NativeMeta>,
  start: SystemTime,
}

impl NativeAssertNe {
  pub fn new() -> Self {
    Self {
      meta: Box::new(NativeMeta {
        name: "assertNe".to_string(),
        arity: 2,
      }),
      start: SystemTime::now(),
    }
  }
}

impl NativeFun for NativeAssertNe {
  fn meta(&self) -> &NativeMeta {
    &self.meta
  }

  fn eq(&self, rhs: &dyn NativeFun) -> bool {
    native_eq(self, rhs)
  }

  fn call(&self, args: &[Value]) -> NativeResult {
    if args[0] != args[1] {
      return NativeResult::Success(Value::Nil);
    }

    NativeResult::RuntimeError(format!("{:?} and {:?} where equal", args[0], args[1]))
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[cfg(test)]
  mod clock {
    use super::*;

    #[test]
    fn new() {
      let clock = NativeClock::new();

      assert_eq!(clock.meta.name, "clock");
      assert_eq!(clock.meta.arity, 0);
    }

    #[test]
    fn call() {
      let clock = NativeClock::new();
      let values: &[Value] = &[];

      let result1 = clock.call(values);
      let res1 = match result1 {
        NativeResult::Success(res) => res,
        NativeResult::RuntimeError(_) => panic!(),
      };

      let result2 = clock.call(values);
      let res2 = match result2 {
        NativeResult::Success(res) => res,
        NativeResult::RuntimeError(_) => panic!(),
      };

      match (res1, res2) {
        (Value::Number(num1), Value::Number(num2)) => assert!(num1 <= num2),
        _ => panic!(),
      }
    }
  }
}