mod stderr;
mod stdin;
mod stdout;

use laythe_core::{hooks::GcHooks, module::{Module, Package}, utils::IdEmitter};
use laythe_env::managed::Gc;
use std::path::PathBuf;
use stderr::{declare_stderr, define_stderr};
use stdin::{declare_stdin, define_stdin};
use stdout::{declare_stdout, define_stdout};

use crate::StdResult;

const STDIO_PATH: &str = "std/io/stdio";

pub fn stdio_module(
  hooks: &GcHooks,
  std: &Package,
  emitter: &mut IdEmitter,
) -> StdResult<Gc<Module>> {
  let mut module = hooks.manage(Module::from_path(
    &hooks,
    PathBuf::from(STDIO_PATH),
    emitter.emit(),
  )?);

  declare_stderr(hooks, &mut module, std)?;
  declare_stdin(hooks, &mut module, std)?;
  declare_stdout(hooks, &mut module, std)?;

  define_stderr(hooks, &module, &*std)?;
  define_stdin(hooks, &module, &*std)?;
  define_stdout(hooks, &module, &*std)?;

  Ok(module)
}
