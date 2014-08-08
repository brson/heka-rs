#![feature(globs)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

extern crate protobuf; // depend on rust-protobuf runtime
extern crate libc;

use std::path::Path;
use std::io::fs::File;
use std::io::BufferedReader;

mod message; // add generated file to the project
mod sandbox;

fn main() {
	let mut count = 0u;
    let mut sb = sandbox::LuaSandbox::new("../test/nginx_access.lua".as_bytes(), "heka_rs/modules".as_bytes(), 8*1024*1024, 1000000, 1024*63);
    let mut m = message::HekaMessage::new();

    let r = sb.init("".as_bytes());
    if r != 0 {
        println!("init failed {}", r);
    }
	let path : Path   = Path::new("../test/access.log");
	let mut file = BufferedReader::new(File::open(&path));
	for line in file.lines() {
		count = count + 1;
		m.set_payload(line.unwrap());
        // todo what we really want is const reference passed to process_message 
        // but the sandbox has to hold it outside the scope of process_message
        // for the callbacks to access.  It is unclear how I can specify
        // the lifetime to make the compiler happy (so it is cloned for now)
		let rc = sb.process_message(&m);
		if rc > 0 {
			println!("process message failed {}", rc);
			break;
        } else if rc == -1 {
			println!("process message failed parsing line {}: {}", count, m.get_payload());
        }
	}

    let err = sb.last_error();
    if !err.is_empty() {
        println!("last error {}", err);
    }
    sb.destroy("".as_bytes());
    println!("count: {}", count);
}
