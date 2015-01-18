#![allow(unstable)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![feature(plugin)]

extern crate libc;
extern crate protobuf;
extern crate regex;
#[plugin]
extern crate regex_macros;
extern crate uuid;

pub mod message;
pub mod sandbox;
pub mod splitter;
