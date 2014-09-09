#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![feature(phase)]

extern crate libc;
extern crate protobuf;
extern crate regex;
#[phase(plugin)]
extern crate regex_macros;
extern crate uuid;

pub mod message;
pub mod sandbox;
pub mod splitter;
