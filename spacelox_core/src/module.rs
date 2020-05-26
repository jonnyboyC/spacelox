use crate::{
  hooks::Hooks,
  managed::{Manage, Managed, Trace},
  value::Value,
  ModuleResult, SlHashMap,
};
use hashbrown::{hash_map::Entry, HashMap};
use std::fmt;
use std::mem;

/// A struct representing a collection of class functions and variable of shared functionality
#[derive(Clone)]
pub struct Module {
  /// The name of the module
  pub name: Managed<String>,

  /// A key value set of named exports from the provided modules
  exports: SlHashMap<Managed<String>, Value>,

  /// All of the top level symbols in this module
  symbols: SlHashMap<Managed<String>, Value>,
}

impl Module {
  /// Create a new spacelox module
  ///
  /// # Example
  /// ```
  /// use spacelox_core::module::Module;
  /// use spacelox_core::memory::{Gc, NO_GC};
  ///
  /// let gc = Gc::default();
  /// let module = Module::new(gc.manage_str("example".to_string(), &NO_GC));
  /// ```
  pub fn new(name: Managed<String>) -> Self {
    Module {
      name,
      exports: HashMap::with_hasher(Default::default()),
      symbols: HashMap::with_hasher(Default::default()),
    }
  }

  /// Add export a new symbol from this module. Exported names must be unique
  ///
  /// # Example
  /// ```
  /// use spacelox_core::module::Module;
  /// use spacelox_core::memory::{Gc};
  /// use spacelox_core::value::Value;
  /// use spacelox_core::hooks::{NoContext, Hooks, HookContext};
  ///
  /// let gc = Gc::default();
  /// let mut context = NoContext::new(&gc);
  /// let hooks = Hooks::new(&mut context);
  ///
  /// let mut module = Module::new(hooks.manage_str("module".to_string()));
  ///
  /// let export_name = hooks.manage_str("exported".to_string());
  ///
  /// let result1 = module.export_symbol(&hooks, export_name, Value::from(true));
  /// let result2 = module.export_symbol(&hooks, export_name, Value::from(false));
  ///
  /// assert_eq!(result1.is_ok(), true);
  /// assert_eq!(result2.is_err(), true);
  /// ```
  pub fn export_symbol(
    &mut self,
    hooks: &Hooks,
    name: Managed<String>,
    symbol: Value,
  ) -> ModuleResult<()> {
    match self.exports.entry(name) {
      Entry::Occupied(_) => Err(hooks.make_error(format!(
        "{} has already been exported from {}",
        name, self.name
      ))),
      Entry::Vacant(entry) => {
        entry.insert(symbol);
        Ok(())
      }
    }
  }

  /// Get a reference to all exported symbols in this module
  ///
  /// # Example
  /// ```
  /// use spacelox_core::module::Module;
  /// use spacelox_core::memory::{Gc};
  /// use spacelox_core::value::Value;
  /// use spacelox_core::hooks::{NoContext, Hooks, HookContext};
  ///
  /// let gc = Gc::default();
  /// let mut context = NoContext::new(&gc);
  /// let hooks = Hooks::new(&mut context);
  ///
  /// let mut module = Module::new(hooks.manage_str("module".to_string()));
  ///
  /// let export_name = hooks.manage_str("exported".to_string());
  /// module.export_symbol(&hooks, export_name, Value::from(true));
  ///
  /// let symbols = module.import();
  ///
  /// assert_eq!(symbols.len(), 1);
  ///
  /// if let Some(result) = symbols.get(&export_name) {
  ///   assert_eq!(*result, Value::from(true));
  /// } else {
  ///   assert!(false);
  /// }
  /// ```
  pub fn import(&self) -> &SlHashMap<Managed<String>, Value> {
    &self.exports
  }

  /// Insert a symbol into this module's symbol table
  ///
  /// #Examples
  /// ```
  /// use spacelox_core::module::Module;
  /// use spacelox_core::memory::{Gc};
  /// use spacelox_core::value::Value;
  /// use spacelox_core::hooks::{NoContext, Hooks, HookContext};
  ///
  /// let gc = Gc::default();
  /// let mut context = NoContext::new(&gc);
  /// let hooks = Hooks::new(&mut context);
  ///
  /// let mut module = Module::new(hooks.manage_str("module".to_string()));
  ///
  /// let name = hooks.manage_str("exported".to_string());
  /// module.insert_symbol(name, Value::from(true));
  ///
  /// let symbol = module.get_symbol(name);
  ///
  /// if let Some(result) = symbol {
  ///   assert_eq!(*result, Value::from(true));
  /// } else {
  ///   assert!(false);
  /// }
  /// ```
  pub fn insert_symbol(&mut self, name: Managed<String>, symbol: Value) {
    self.symbols.insert(name, symbol);
  }

  /// Get a symbol from this module's symbol table
  ///
  /// #Examples
  /// ```
  /// use spacelox_core::module::Module;
  /// use spacelox_core::memory::{Gc};
  /// use spacelox_core::value::Value;
  /// use spacelox_core::hooks::{NoContext, Hooks, HookContext};
  ///
  /// let gc = Gc::default();
  /// let mut context = NoContext::new(&gc);
  /// let hooks = Hooks::new(&mut context);
  ///
  /// let mut module = Module::new(hooks.manage_str("module".to_string()));
  ///
  /// let name = hooks.manage_str("exported".to_string());
  ///
  /// let symbol = module.get_symbol(name);
  ///
  /// if let Some(result) = symbol {
  ///   assert!(false);
  /// } else {
  ///   assert!(true);
  /// }
  /// ```
  pub fn get_symbol(&mut self, name: Managed<String>) -> Option<&Value> {
    self.symbols.get(&name)
  }
}

impl fmt::Debug for Module {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    f.debug_struct("Module")
      .field("name", &*self.name)
      .field("exports", &"SlHashMap: { ... }")
      .field("symbols", &"SlHashMap: { ... }")
      .finish()
  }
}

impl Trace for Module {
  fn trace(&self) -> bool {
    self.name.trace();

    self.exports.iter().for_each(|(key, value)| {
      key.trace();
      value.trace();
    });
    self.symbols.iter().for_each(|(key, value)| {
      key.trace();
      value.trace();
    });

    true
  }
  fn trace_debug(&self, stdio: &dyn crate::io::StdIo) -> bool {
    self.name.trace_debug(stdio);

    self.exports.iter().for_each(|(key, value)| {
      key.trace_debug(stdio);
      value.trace_debug(stdio);
    });
    self.symbols.iter().for_each(|(key, value)| {
      key.trace_debug(stdio);
      value.trace_debug(stdio);
    });

    true
  }
}

impl Manage for Module {
  fn alloc_type(&self) -> &str {
    "module"
  }
  fn debug(&self) -> String {
    format!("{:?}", self)
  }
  fn debug_free(&self) -> String {
    "Module: {{ name: {{...}}, exports: {{...}}, symbols: {{...}}}}".to_string()
  }

  fn size(&self) -> usize {
    mem::size_of::<Self>()
      + (mem::size_of::<Managed<String>>() + mem::size_of::<Value>())
        * (self.exports.capacity() + self.symbols.capacity())
  }
}
