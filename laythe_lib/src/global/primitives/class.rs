use crate::native;
use laythe_core::{
  hooks::{GcHooks, Hooks},
  managed::{GcObj, Trace},
  object::{Class, LyNative, Native, NativeMetaBuilder},
  signature::Arity,
  val,
  value::{Value, VALUE_NIL},
  Call,
};
use std::io::Write;

pub const CLASS_CLASS_NAME: &str = "Class";

const CLASS_SUPER_CLS: NativeMetaBuilder = NativeMetaBuilder::method("superCls", Arity::Fixed(0));
const CLASS_STR: NativeMetaBuilder = NativeMetaBuilder::method("str", Arity::Fixed(0));
const CLASS_NAME: NativeMetaBuilder = NativeMetaBuilder::method("name", Arity::Fixed(0));

pub fn create_class_class(hooks: &GcHooks, object: GcObj<Class>) -> GcObj<Class> {
  let name = hooks.manage_str(CLASS_CLASS_NAME);
  let mut class = hooks.manage_obj(Class::bare(name));
  class.inherit(hooks, object);

  class.add_method(
    &hooks,
    hooks.manage_str(CLASS_SUPER_CLS.name),
    val!(ClassSuperCls::native(hooks)),
  );

  class.add_method(
    &hooks,
    hooks.manage_str(CLASS_STR.name),
    val!(ClassStr::native(hooks)),
  );

  class.add_method(
    &hooks,
    hooks.manage_str(CLASS_NAME.name),
    val!(ClassName::native(hooks)),
  );

  class
}

native!(ClassSuperCls, CLASS_SUPER_CLS);

impl LyNative for ClassSuperCls {
  fn call(&self, _hooks: &mut Hooks, this: Option<Value>, _args: &[Value]) -> Call {
    let super_class = this
      .unwrap()
      .to_obj()
      .to_class()
      .super_class()
      .map(Value::from)
      .unwrap_or(VALUE_NIL);

    Call::Ok(super_class)
  }
}

native!(ClassStr, CLASS_STR);

impl LyNative for ClassStr {
  fn call(&self, hooks: &mut Hooks, this: Option<Value>, _args: &[Value]) -> Call {
    let class = this.unwrap().to_obj().to_class();

    Call::Ok(val!(hooks.manage_str(&format!(
      "<class {} {:p}>",
      &*class.name(),
      &*class
    ))))
  }
}

native!(ClassName, CLASS_NAME);

impl LyNative for ClassName {
  fn call(&self, _hooks: &mut Hooks, this: Option<Value>, _args: &[Value]) -> Call {
    let class = this.unwrap().to_obj().to_class();
    Call::Ok(val!(class.name()))
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::support::MockedContext;

  mod super_cls {
    use super::*;

    #[test]
    fn new() {
      let mut context = MockedContext::default();
      let hooks = GcHooks::new(&mut context);

      let class_super_class = ClassSuperCls::native(&hooks);

      assert_eq!(class_super_class.meta().name, "superCls");
      assert_eq!(class_super_class.meta().signature.arity, Arity::Fixed(0));
    }

    #[test]
    fn call() {
      let mut context = MockedContext::default();
      let mut hooks = Hooks::new(&mut context);
      let class_super_class = ClassSuperCls::native(&hooks.as_gc());

      let mut class = hooks.manage_obj(Class::bare(hooks.manage_str("someClass")));

      let super_class = hooks.manage_obj(Class::bare(hooks.manage_str("someSuperClass")));

      class.inherit(&hooks.as_gc(), super_class);

      let class_value = val!(class);
      let super_class_value = val!(super_class);

      let result1 = class_super_class.call(&mut hooks, Some(class_value), &[]);
      let result2 = class_super_class.call(&mut hooks, Some(super_class_value), &[]);

      match result1 {
        Call::Ok(r) => assert_eq!(r, super_class_value),
        _ => assert!(false),
      }
      match result2 {
        Call::Ok(r) => assert_eq!(r, VALUE_NIL),
        _ => assert!(false),
      }
    }
  }

  mod str {
    use super::*;

    #[test]
    fn new() {
      let mut context = MockedContext::default();
      let hooks = GcHooks::new(&mut context);

      let class_str = ClassStr::native(&hooks);

      assert_eq!(class_str.meta().name, "str");
      assert_eq!(class_str.meta().signature.arity, Arity::Fixed(0));
    }

    #[test]
    fn call() {
      let mut context = MockedContext::default();
      let mut hooks = Hooks::new(&mut context);
      let class_str = ClassStr::native(&hooks.as_gc());

      let class = hooks.manage_obj(Class::bare(hooks.manage_str("someClass".to_string())));

      let class_value = val!(class);

      let result = class_str.call(&mut hooks, Some(class_value), &[]).unwrap();
      assert!(result.to_obj().to_str().contains("<class someClass"));
    }
  }

  mod name {
    use super::*;

    #[test]
    fn new() {
      let mut context = MockedContext::default();
      let hooks = GcHooks::new(&mut context);

      let class_str = ClassName::native(&hooks);

      assert_eq!(class_str.meta().name, "name");
      assert_eq!(class_str.meta().signature.arity, Arity::Fixed(0));
    }

    #[test]
    fn call() {
      let mut context = MockedContext::default();
      let mut hooks = Hooks::new(&mut context);
      let class_str = ClassName::native(&hooks.as_gc());

      let class = hooks.manage_obj(Class::bare(hooks.manage_str("someClass".to_string())));

      let class_value = val!(class);

      let result = class_str.call(&mut hooks, Some(class_value), &[]).unwrap();
      assert_eq!(result.to_obj().to_str(), "someClass");
    }
  }
}
