use crate::lua_env;

use lua_env::LEnv;
use mlua::prelude::*;
use mlua::Thread;
use rand::Rng;

enum Syscall {
  Read,
  Write(String),
  // (func, args)
  Spawn(String, String),
}

#[derive(Clone, Debug)]
struct Process<'a> {
  thread: Thread<'a>,
  retval: Option<String>,
}

impl<'a> Process<'a> {
  fn new(thread: Thread<'a>) -> Self {
    Self {
      thread,
      retval: None,
    }
  }

  fn step(&self) -> Result<Syscall, LuaError> {
    let values = self.thread.resume::<_, LuaTable>(self.retval.clone())?;
    let action = values.get::<_, LuaString>(1)?;
    let args = values.get::<_, LuaTable>(2)?;
    match action.to_str() {
      Ok("read") => return Ok(Syscall::Read),
      Ok("write") => return Ok(Syscall::Write(args.get::<_, String>(1)?)),
      Ok("spawn") => {
        let func = args.get::<_, String>(1)?;
        let args = args.get::<_, String>(2)?;
        return Ok(Syscall::Spawn(func, args));
      },
      _ => panic!("unknown action"),
    }
  }
}

pub struct OS<'a> {
  procs: Vec<Process<'a>>,
  buffer: String,
}

impl<'a> OS<'a> {
  pub fn new(lua_env: &'a LEnv) -> Result<OS, LuaError> {
    if let Ok(_) = lua_env.exec() {
      let co_main: Thread = lua_env.globals().get("co_main")?;
      let main_process = Process::new(co_main);
      Ok(OS {
        procs: vec![main_process],
        buffer: String::new(),
      })
    } else {
      panic!("can't exec lua code")
    }
  }

  pub fn run(&mut self, lua_env: &'a LEnv) -> Result<(), LuaError> {
    let mut rng = rand::thread_rng();
    while !self.procs.is_empty() {
      let index = rng.gen_range(0..self.procs.len());
      let mut current = self.procs.remove(index);
      if let Ok(syscall) = current.step() {
        match syscall {
          Syscall::Read => {
            let retval = rand::thread_rng().gen_range(0..2);
            current.retval = Some(format!("{retval}"));
          },
          Syscall::Write(s) => {
            self.buffer.push_str(s.as_str());
          },
          Syscall::Spawn(func_name, args) => {
            let thread = lua_env.create_thread(func_name)?;
            let mut p = Process::new(thread);
            p.retval = Some(args);
            self.procs.push(p);
          },
        }
        self.procs.insert(index, current);
      }
    }
    Ok(())
  }
}
