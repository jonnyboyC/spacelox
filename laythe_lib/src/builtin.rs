use std::io::Write;

use laythe_core::{
  hooks::GcHooks,
  module::Module,
  object::Class,
  value::{Value, ValueKind},
};
use laythe_env::managed::{Managed, Trace};

use crate::global::{BOOL_CLASS_NAME, CLASS_CLASS_NAME, CLOSURE_CLASS_NAME, EXPORT_ERROR_NAME, IMPORT_ERROR_NAME, ITER_CLASS_NAME, LIST_CLASS_NAME, MAP_CLASS_NAME, METHOD_CLASS_NAME, METHOD_NOT_FOUND_ERROR_NAME, MODULE_CLASS_NAME, NATIVE_CLASS_NAME, NIL_CLASS_NAME, NUMBER_CLASS_NAME, OBJECT_CLASS_NAME, PROPERTY_ERROR_NAME, RUNTIME_ERROR_NAME, STRING_CLASS_NAME};

pub struct BuiltIn {
  /// built in classes related to dependencies
  pub dependencies: BuiltInDependencies,

  /// built in classes related to primitives
  pub primitives: BuiltInPrimitives,

  /// built in error classes
  pub errors: BuiltInErrors,
}

impl Trace for BuiltIn {
  fn trace(&self) -> bool {
    self.primitives.trace();
    self.dependencies.trace();
    self.errors.trace()
  }

  fn trace_debug(&self, stdio: &mut dyn Write) -> bool {
    self.primitives.trace_debug(stdio);
    self.dependencies.trace_debug(stdio);
    self.errors.trace_debug(stdio)
  }
}

pub struct BuiltInDependencies {
  /// The
  pub module: Managed<Class>,
}

impl Trace for BuiltInDependencies {
  fn trace(&self) -> bool {
    self.module.trace()
  }

  fn trace_debug(&self, log: &mut dyn Write) -> bool {
    self.module.trace_debug(log)
  }
}

pub struct BuiltInPrimitives {
  /// The base Object class
  pub object: Managed<Class>,

  /// the Nil class
  pub nil: Managed<Class>,

  /// the Bool class
  pub bool: Managed<Class>,

  /// the Class class
  pub class: Managed<Class>,

  /// the Number class
  pub number: Managed<Class>,

  /// the String class
  pub string: Managed<Class>,

  /// the List class
  pub list: Managed<Class>,

  /// the Map class
  pub map: Managed<Class>,

  /// the Iter class
  pub iter: Managed<Class>,

  /// the Closure class
  pub closure: Managed<Class>,

  /// the method class
  pub method: Managed<Class>,

  /// the NativeFun class
  pub native_fun: Managed<Class>,
}

impl BuiltInPrimitives {
  pub fn for_value(&self, value: Value, kind: ValueKind) -> Managed<Class> {
    match kind {
      ValueKind::Bool => self.bool,
      ValueKind::Nil => self.nil,
      ValueKind::Number => self.number,
      ValueKind::String => self.string,
      ValueKind::List => self.list,
      ValueKind::Map => self.map,
      ValueKind::Fun => panic!(),
      ValueKind::Closure => self.closure,
      ValueKind::Class => value.to_class().meta().expect("Meta class not set."),
      ValueKind::Instance => value.to_instance().class,
      ValueKind::Iter => self.iter,
      ValueKind::Method => self.method,
      ValueKind::Native => self.native_fun,
      ValueKind::Upvalue => {
        let value = value.to_upvalue().value();
        self.for_value(value, value.kind())
      }
    }
  }
}

impl Trace for BuiltInPrimitives {
  fn trace(&self) -> bool {
    self.bool.trace();
    self.nil.trace();
    self.class.trace();
    self.number.trace();
    self.string.trace();
    self.list.trace();
    self.iter.trace();
    self.map.trace();
    self.closure.trace();
    self.method.trace();
    self.native_fun.trace();

    true
  }

  fn trace_debug(&self, stdio: &mut dyn Write) -> bool {
    self.bool.trace_debug(stdio);
    self.nil.trace_debug(stdio);
    self.class.trace_debug(stdio);
    self.number.trace_debug(stdio);
    self.string.trace_debug(stdio);
    self.list.trace_debug(stdio);
    self.iter.trace_debug(stdio);
    self.map.trace_debug(stdio);
    self.closure.trace_debug(stdio);
    self.method.trace_debug(stdio);
    self.native_fun.trace_debug(stdio);

    true
  }
}

pub struct BuiltInErrors {
  pub runtime: Managed<Class>,

  pub method_not_found: Managed<Class>,

  pub property: Managed<Class>,

  pub import: Managed<Class>,

  pub export: Managed<Class>,
}

impl Trace for BuiltInErrors {
  fn trace(&self) -> bool {
    self.method_not_found.trace()
  }

  fn trace_debug(&self, log: &mut dyn Write) -> bool {
    self.method_not_found.trace_debug(log)
  }
}

pub fn builtin_from_module(hooks: &GcHooks, module: &Module) -> Option<BuiltIn> {
  Some(BuiltIn {
    primitives: BuiltInPrimitives {
      object: module
        .get_symbol(hooks.manage_str(OBJECT_CLASS_NAME))?
        .to_class(),
      nil: module
        .get_symbol(hooks.manage_str(NIL_CLASS_NAME))?
        .to_class(),
      bool: module
        .get_symbol(hooks.manage_str(BOOL_CLASS_NAME))?
        .to_class(),
      class: module
        .get_symbol(hooks.manage_str(CLASS_CLASS_NAME))?
        .to_class(),
      number: module
        .get_symbol(hooks.manage_str(NUMBER_CLASS_NAME))?
        .to_class(),
      string: module
        .get_symbol(hooks.manage_str(STRING_CLASS_NAME))?
        .to_class(),
      list: module
        .get_symbol(hooks.manage_str(LIST_CLASS_NAME))?
        .to_class(),
      map: module
        .get_symbol(hooks.manage_str(MAP_CLASS_NAME))?
        .to_class(),
      iter: module
        .get_symbol(hooks.manage_str(ITER_CLASS_NAME))?
        .to_class(),
      closure: module
        .get_symbol(hooks.manage_str(CLOSURE_CLASS_NAME))?
        .to_class(),
      method: module
        .get_symbol(hooks.manage_str(METHOD_CLASS_NAME))?
        .to_class(),
      native_fun: module
        .get_symbol(hooks.manage_str(NATIVE_CLASS_NAME))?
        .to_class(),
    },
    dependencies: BuiltInDependencies {
      module: module
        .get_symbol(hooks.manage_str(MODULE_CLASS_NAME))?
        .to_class(),
    },
    errors: BuiltInErrors {
      runtime: module
        .get_symbol(hooks.manage_str(RUNTIME_ERROR_NAME))?
        .to_class(),
      method_not_found: module
        .get_symbol(hooks.manage_str(METHOD_NOT_FOUND_ERROR_NAME))?
        .to_class(),
      property: module
        .get_symbol(hooks.manage_str(PROPERTY_ERROR_NAME))?
        .to_class(),
      import: module
        .get_symbol(hooks.manage_str(IMPORT_ERROR_NAME))?
        .to_class(),
      export: module
        .get_symbol(hooks.manage_str(EXPORT_ERROR_NAME))?
        .to_class(),
    },
  })
}