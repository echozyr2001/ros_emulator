#![allow(dead_code)]

mod lua_env;

use lua_env::LEnv;
use mlua::prelude::*;
use mlua::Thread;
use rand::Rng;

enum Syscall {
  Read,
  Write(String),
  Spawn(String, Vec<String>), // (func_name, args)
}

struct OS<'a> {
  procs: Vec<Process<'a>>,
  buffer: String,
}

impl<'a> OS<'a> {
  fn new(lua_env: &'a LEnv) -> Self {
    if let Err(e) = lua_env.exec() {
      panic!("{e}");
    }
    let co_main: Thread = lua_env.globals().get("co_main").unwrap();
    let main_process = Process::new(co_main);
    OS {
      procs: vec![main_process],
      buffer: String::new(),
    }
  }

  fn run(&mut self, lua_env: &'a LEnv) -> Result<(), LuaError> {
    let mut rng = rand::thread_rng();
    // while the process queue is not empty
    while !self.procs.is_empty() {
      // choose a random index
      let index = rng.gen_range(0..self.procs.len());
      // remove and get the process instance
      let mut current = self.procs.remove(index);
      // do step and get syscall and args
      let syscall = current.step()?;
      // match the syscall
      match syscall {
        Syscall::Read => {
          let retval = rand::thread_rng().gen_range(0..2);
          current.retval = Some(retval);
        },
        Syscall::Write(s) => {
          self.buffer.push_str(s.as_str());
        },
        Syscall::Spawn(func_name, args) => {
          // create a thread
          let thread = lua_env.create_thread(func_name)?;
          // create a process
          let p = Process::new(thread);
          self.procs.push(p);
          println!("todo");
        },
      }
      // TODO
      // put process back if it is not dead
      self.procs.insert(index, current);
    }
    Ok(())
  }
}

#[derive(Clone, PartialEq, Debug)]
struct Process<'a> {
  thread: Thread<'a>,
  retval: Option<u8>,
}

impl<'a> Process<'a> {
  fn new(thread: Thread<'a>) -> Self {
    Self {
      thread,
      retval: None,
    }
  }

  fn step(&self) -> Result<Syscall, LuaError> {
    /*
    function process(name)
      coroutine.yield({"write", {name}})
    end
    function main()
      local a = coroutine.yield({"read", {}})
      print(a)
      coroutine.yield({"spawn", {process, "A"}})
      coroutine.yield({"spawn", {process, "A"}})
    end
    co_main = coroutine.create(main)
    */
    // <参数，返回值>
    // 返回的就是yield中的内容
    // 👇 { "spawn", {process, "A"} }
    let values = self.thread.resume::<_, LuaTable>(self.retval)?;

    // 👇 "spawn"
    let action = values.get::<_, LuaString>(1)?;
    // 👇 {process, "A"}
    let args = values.get::<_, LuaTable>(2)?;

    match action.to_str() {
      // 👇 { "read", {} }
      Ok("read") => return Ok(Syscall::Read),
      // 👇 { "write", { name } }
      Ok("write") => return Ok(Syscall::Write(args.get::<_, String>(1)?)),
      // 👇 { "spawn", {process, "A"} }
      Ok("spawn") => {
        // 👇 process
        let func = args.get::<_, String>(1)?;
        let args = args
          .sequence_values::<String>()
          .skip(1)
          .collect::<Result<Vec<_>, _>>()?;
        return Ok(Syscall::Spawn(func, args));
      },
      _ => panic!("unknown action"),
    }
  }
}

fn main() -> Result<(), LuaError> {
  let lua_code = r#"
    function process(name)
      sys_write(name) 
    end
    function main()
      local a = sys_read()
      print(a)
      sys_spawn(process, "A")
      sys_spawn(process, "A")
    end
  "#;

  let lua_env = LEnv::new(lua_code);
  println!("{}", lua_env.src);

  let mut os = OS::new(&lua_env);
  os.run(&lua_env)?;
  Ok(())
}
