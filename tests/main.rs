use mlua::prelude::*;

use ros_emulator::lua_env::LEnv;
use ros_emulator::os::OS;
use ros_emulator::utils::get_code_from_file;

#[test]
fn test_read_file() -> Result<(), LuaError> {
  let lua_code = get_code_from_file("tests/data/test.lua").unwrap();
  let lua_env = LEnv::new(&lua_code);
  println!("{:?}", lua_env.src);

  let mut os = OS::new(&lua_env)?;
  os.run(&lua_env)?;

  Ok(())
}
