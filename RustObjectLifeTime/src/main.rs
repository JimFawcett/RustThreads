/////////////////////////////////////////////////////////////
// rust_object_lifetime::main.rs                           //
//                                                         //
// Jim Fawcett, https://JimFawcett.github.io, 10 May 2020  //
/////////////////////////////////////////////////////////////

#![allow(dead_code)]

use std::thread;
use std::io::prelude::*;

#[derive(Debug)]
struct LifeTime {
    name:String,
}
impl Drop for LifeTime {
    fn drop(&mut self) {
        print!("\n  LifeTime instance {:?} dropping on {:?}", self.name, thread_id());
    }
}
impl LifeTime {
    fn new(n:&str) -> Self {
        print!("\n  {:?} created on {:?}", n, thread_id());
        Self {
            name: n.to_string(),
        }
    }
    fn get_name(&self) -> &str {
        &self.name
    }
    fn set_name(&mut self, n:&str) {
        print!("\n  setting name {:?} on {:?}", n, thread_id());
        self.name = n.to_string();
    }
}
fn test1() {
    let _handle:thread::JoinHandle<()>;
    {
        let mut _heapy = Box::new(LifeTime::new("Heapy"));
        let mut _stacky = LifeTime::new("stacky");

        _handle = thread::spawn(move || {
            print!("\n  starting {:?}", thread_id());
            _heapy.set_name("_heapy in thread");
            print!("\n  {:?} leaving thread scope", thread_id());
            std::io::stdout().flush().unwrap();
        });

        print!("\n  {:?} waiting on join", thread_id());
        std::io::stdout().flush().unwrap();
        _handle.join().unwrap();
        print!("\n  {:?} returned from join", thread_id());
        std::io::stdout().flush().unwrap();
    }
}
fn db_show<T:std::fmt::Debug>(t:T, msg:&str, p:bool) {
    print!("\n  --{}", msg);
    if p {
        let name = std::any::type_name::<T>();
        print!("\n  --TypeId: {},\n  --size: {}", name, std::mem::size_of::<T>());
    }
    print!("\n  --{:?}", t);
} 
fn thread_id() -> thread::ThreadId {
    thread::current().id()
}
fn thread_id_value(id:thread::ThreadId) -> char {
    let sid = format!("{:?}", id);
    sid.chars().nth(9).unwrap()
}
fn main() -> std::io::Result<()> {

    test1();
    println!("\n\n  That's all Folks!\n\n");
    Ok(())
}
