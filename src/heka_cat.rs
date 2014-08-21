#![feature(globs)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

extern crate std;
extern crate protobuf; // depend on rust-protobuf runtime
extern crate getopts;
extern crate libc;

use std::path::Path;
use std::io::fs::File;
use std::io::BufReader;
use std::os;
use getopts::{optopt,optflag,getopts,OptGroup};

use protobuf::clear::Clear;
use protobuf::Message;
use message::pb;

mod message; // add generated file to the project
mod sandbox;
mod splitter;

fn print_usage(program: &str, _opts: &[OptGroup]) {
    println!("{}", getopts::usage(format!("Usage: {} [options] <input_file>", program).as_slice(), _opts))
}

fn main() {
    let args: Vec<String> = os::args();
    let program = args[0].clone();
    let opts = [
        optopt("m", "match", "set the message matcher filter", "TRUE"),
        optopt("p", "plugin", "set plugin name (will look for the toml in the same location)", "plugin.lua"),
        optopt("o", "output", "set output file name", "heka_cat.hpb"),
        optflag("h", "help", "print this help menu")
    ];
    let matches = match getopts(args.tail(), opts) {
        Ok(m) => { m }
        Err(f) => {
            println!("{}\n", f.to_string());
            print_usage(program.as_slice(), opts);
            return;
        }
    };
    if matches.opt_present("h") {
        print_usage(program.as_slice(), opts);
        return;
    }
    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(program.as_slice(), opts);
        return;
    };

    let path : Path = Path::new(input);
    let mut hps = splitter::HekaProtobufStream::new(File::open(&path), 1024*64+255+3); // max message size + header + seperators
    let mut lsb = sandbox::LuaSandbox::new("../test/fxa_active_daily_users.lua".as_bytes(), "heka_rs/modules".as_bytes(), 0, 0, 0);
    let preservation = "fxa_active_daily_users.preserve";
    let r = match Path::new(preservation).stat() {
        Ok(_) => lsb.init(preservation.as_bytes()),
        Err(_) => lsb.init("".as_bytes())
    };
    if r != 0 {
        fail!("sandbox_init failed {} {}", r, lsb.last_error());
    }

    let mut count = 0u;
    let mut match_count = 0u;
    let mut msg = Some(pb::HekaMessage::new());
    loop {
        match hps.read_next() {
            Ok(m) => {
                if m.is_some() {
                    let m = m.unwrap();
                    count += 1;
                    let mut reader = BufReader::new(m);
                    let mut cis = protobuf::CodedInputStream::new(&mut reader);
                    msg.get_mut_ref().clear();
                    msg.get_mut_ref().merge_from(&mut cis); // todo: warning this asserts on corrupt records
                    if msg.get_ref().is_initialized() {
                        if msg.get_ref().get_logger() == "FxaAuth"
                        && msg.get_ref().get_field_type() == "request.summary"
                        && message::match_field(msg.get_ref(), "path", 0, 0, "/v1/certificate/sign")
                        && message::match_field_numeric(msg.get_ref(), "errno", 0, 0, 0f64)
                        { // todo implement matcher
                            match_count += 1;
                            let (rc, mm) = lsb.process_message(msg.take_unwrap());
                            msg = Some(mm);
                            if rc > 0 {
                                println!("process message failed {} {}", rc, lsb.last_error());
                                return;
                            } else if rc == -1 {
                                println!("process message failed parsing line {}: {}", count, msg.get_ref().get_payload());
                            }
                        }
                    } else {
                        println!("missing required field");
                    }
                }
            }
            Err(e) => {
                match e.kind {
                    std::io::EndOfFile => { break; }
                    std::io::OtherIoError => { println!("non-fatal read error: {}", e); }
                    _ => {
                        println!("read error: {}", e);
                        break;
                    }
                }
            }
        }
    }
    let err = lsb.last_error();
    if !err.is_empty() {
        println!("last error {}", err);
    }
    lsb.destroy(preservation.as_bytes());
    println!("offset: {} total_count: {} match_count: {}", hps.tell().unwrap_or(0), count, match_count);
}
