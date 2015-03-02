// build.rs
// Right now this is more or less a literal translation of build.sh.
// Something needs to be done about protobuf code generation (which itself has
// a rust dependency).
#![allow(unstable)]

use std::old_io::Command;
use std::old_io::process::ProcessExit;
use std::os;
use std::old_path::Path;
// original script is *NIX-only
use std::old_path::posix::SEP;

fn main() {
    let out_dir = os::getenv("OUT_DIR").unwrap();
    let out_path = Path::new(out_dir.as_slice());
    let root_path = os::getcwd().unwrap();

    os::change_dir(&out_path).unwrap();

    let cmake_status = match Command::new("cmake").arg(
        "-DCMAKE_BUILD_TYPE=release").arg(root_path).status() {
        Ok(status) => status,
        Err(e) => panic!("failed to execute process: {}", e),
    };

    let cmake_ret = match cmake_status {
        ProcessExit::ExitStatus(i) => i,
        ProcessExit::ExitSignal(_) => 0
    };

    if cmake_ret != 0 {
        os::set_exit_status(cmake_ret);
        return;
    }

    let make_status = match Command::new("make").status() {
        Ok(status) => status,
        Err(e) => panic!("failed to execute process: {}", e),
    };

    let make_ret = match make_status {
        ProcessExit::ExitStatus(i) => i,
        ProcessExit::ExitSignal(_) => 0
    };
    if make_ret != 0 {
        os::set_exit_status(make_ret);
        return;
    }

    println!("cargo:rustc-flags=-L {}{}lib", out_dir, SEP);
}
