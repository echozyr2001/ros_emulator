#![allow(dead_code)]

mod cli;
mod lua_env;
mod os;
mod utils;

use crate::lua_env::LEnv;
use crate::os::OS;
use crate::utils::get_code_from_file;
use mlua::prelude::*;

fn main() -> Result<(), LuaError> {
  let args = cli::parse_args();
  let lua_code = get_code_from_file(&args.file_path)?;
  let lua_env = LEnv::new(&lua_code);

  #[cfg(debug_assertions)]
  println!("{}", lua_env.src);

  let mut os = OS::new(&lua_env)?;
  os.run(&lua_env)?;
  os.print_buffer();
  Ok(())
}
