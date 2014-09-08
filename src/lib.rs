#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![feature(phase)]
#[phase(plugin)]

extern crate regex_macros;
extern crate regex;
extern crate libc;
extern crate protobuf; // depend on rust-protobuf runtime

pub mod message; // add generated file to the project
pub mod sandbox;
pub mod splitter;
