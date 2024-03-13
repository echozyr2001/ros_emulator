use mlua::prelude::*;
use mlua::{Function, Lua, Table, Thread};
use regex::Regex;
pub struct LEnv {
  pub src: String,
  lua: Lua,
}

impl LEnv {
  pub fn new(src: &str) -> Self {
    Self {
      src: format_code(src),
      lua: Lua::new(),
    }
  }

  pub fn exec(&self) -> Result<(), LuaError> {
    self.lua.load(&self.src).exec()
  }

  pub fn globals(&self) -> Table<'_> {
    self.lua.globals()
  }

  // 传入func_name
  pub fn create_thread(&self, func_name: String) -> Result<Thread, LuaError> {
    self
      .lua
      .create_thread(self.lua.globals().get::<_, Function>(func_name)?)
  }
}

fn format_code(src: &str) -> String {
  let re = Regex::new(r"sys_([a-z]+)\(([^)]*)\)").unwrap();

  format!("{}co_main = coroutine.create(main)", {
    re.replace_all(src, |caps: &regex::Captures| {
      let func = &caps[1];
      let args = if caps[2].is_empty() {
        "{}".to_string()
      } else {
        format!("{{{}}}", &caps[2])
      };
      format!("coroutine.yield({{\"{}\", {}}})", func, args)
    })
  })
}
