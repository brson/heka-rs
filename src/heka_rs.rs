#![feature(phase)]
#![feature(globs)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

extern crate std;
extern crate protobuf; // depend on rust-protobuf runtime
extern crate libc;
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

use std::path::Path;
use std::io::fs::File;
use std::io::BufferedReader;
use message::pb;

mod message;
mod sandbox;

fn main() {
    let mut count = 0u;
    let mut sb = sandbox::LuaSandbox::new("../test/nginx_access.lua".as_bytes(), "heka_rs/modules".as_bytes(), 0, 0, 0);
    let mut m = Some(pb::HekaMessage::new());

    let r = sb.init("".as_bytes());
    if r != 0 {
        println!("init failed {}", r);
    }
    let path : Path   = Path::new("../test/access.log");
    let mut file = BufferedReader::new(File::open(&path));
    for line in file.lines() {
        count = count + 1;
        m.as_mut().unwrap().set_payload(line.unwrap());
        let (rc, mm) = sb.process_message(m.take().unwrap());
        m = Some(mm);
        if rc > 0 {
            println!("process message failed {}", rc);
            break;
        } else if rc == -1 {
            println!("process message failed parsing line {}: {}", count, m.as_ref().unwrap().get_payload());
        }
    }

    let err = sb.last_error();
    if !err.is_empty() {
        println!("last error {}", err);
    }
    sb.destroy("".as_bytes());
    println!("count: {}", count);
}
