#![allow(dead_code)]

mod lua_env;
mod os;
mod utils;

use crate::lua_env::LEnv;
use crate::os::OS;
use crate::utils::get_code_from_file;
use mlua::prelude::*;

fn main() -> Result<(), LuaError> {
  let lua_code = get_code_from_file("./test.txt")?;
  let lua_env = LEnv::new(&lua_code);
  println!("{}", lua_env.src);

  let mut os = OS::new(&lua_env)?;
  os.run(&lua_env)?;
  Ok(())
}
