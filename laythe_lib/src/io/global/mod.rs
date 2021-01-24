use laythe_core::{utils::IdEmitter, value::Value};
use laythe_core::{hooks::GcHooks, module::Module, package::Package, val};
use laythe_env::managed::Gc;
use std::path::PathBuf;

use crate::{
  support::{default_error_inheritance, export_and_insert},
  InitResult,
};

pub const ERROR_PATH: &str = "std/io/global.ly";
pub const IO_ERROR: &str = "IoError";

pub fn errors_module(
  hooks: &GcHooks,
  std: &Package,
  emitter: &mut IdEmitter,
) -> InitResult<Gc<Module>> {
  let mut module = hooks.manage(Module::from_path(
    &hooks,
    PathBuf::from(ERROR_PATH),
    emitter.emit(),
  )?);

  declare_io_errors(hooks, &mut module, std)?;
  define_io_errors(hooks, &module, std)?;

  Ok(module)
}

pub fn declare_io_errors(
  hooks: &GcHooks,
  module: &mut Module,
  package: &Package,
) -> InitResult<()> {
  let io_error = default_error_inheritance(hooks, package, IO_ERROR)?;

  export_and_insert(hooks, module, io_error.name(), val!(io_error))
}

pub fn define_io_errors(_hooks: &GcHooks, _module: &Module, _: &Package) -> InitResult<()> {
  Ok(())
}
