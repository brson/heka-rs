extern crate protobuf; // depend on rust-protobuf runtime
extern crate libc;

//trait Sandbox {
//  fn new(source_file: &str, include_path: &str, memory_limit: uint, instruction_limit: uint, output_limit: uint) -> Self;
//  fn init(&mut self, state_file: &[u8]) -> int;
//  fn destroy(&mut self, state_file: &[u8]) -> String
//  fn last_error(&mut self) -> Sting;
//  fn process_message(&mut self, message: msg: &message::HekaMessage) -> int;
//  fn timer_event(&mut self, ns: i64) -> int;
//}

use std;
use message;
use libc::{c_int, c_uint, c_char, size_t, c_longlong, c_void, c_double};

pub enum LSB {}
pub enum LUA {}
pub struct LuaSandbox {
    msg: std::option::Option<message::HekaMessage>,
    lsb: *mut LSB,
}

impl LuaSandbox {

    pub fn new<'a>(lua_file: &[u8],
    require_path: &[u8],
    memory_limit: u32,
    instruction_limit: u32,
    output_limit: u32) -> Box<LuaSandbox> {
        unsafe {
            let mut s = box LuaSandbox{msg: None, lsb: std::ptr::mut_null()};
            lua_file.with_c_str(|lf| {
                require_path.with_c_str(|rp| {
                s.lsb = lsb_create(&s, lf, rp, memory_limit, instruction_limit, output_limit);
                });
            });
            return s;
        }
    }

    fn drop(&mut self) {
        unsafe {
            if self.lsb != std::ptr::mut_null() {
                lsb_destroy(self.lsb, std::ptr::null());
            }
        }
    }

    pub fn init(&mut self, state_file: &[u8]) -> c_int {
        unsafe {
            if self.lsb == std::ptr::mut_null() {
                return -1;
            }
            "inject_message".with_c_str(|f| {lsb_add_function(self.lsb, inject_message, f);});
            "read_message".with_c_str(|f| {lsb_add_function(self.lsb, read_message, f);});

            let mut r: c_int = 0;
            state_file.with_c_str(|sf| {r = lsb_init(self.lsb, sf);}) ;
            return r;
        }
    }

    pub fn destroy(&mut self, state_file: &[u8]) -> String {
        unsafe {
            if self.lsb == std::ptr::mut_null() {
                return String::from_str("not created");
            }
            let mut c: *mut c_char = std::ptr::mut_null();
            state_file.with_c_str(|sf| {
                c = lsb_destroy(self.lsb, sf);
            });
            self.lsb = std::ptr::mut_null();
            if c != std::ptr::mut_null() {
                return std::string::raw::from_buf(c as *const u8);
            } else {
                return String::new();
            }
        }
    }

    pub fn last_error(&mut self) -> String {
        unsafe {
            if self.lsb == std::ptr::mut_null() {
                return String::from_str("not created");
            }
            let c = lsb_get_error(self.lsb);
            if c != std::ptr::null() {
                return std::string::raw::from_buf(c as *const u8);
            } else {
                return String::new();
            }
        }
    }

    pub fn process_message(&mut self, msg: &message::HekaMessage) -> c_int {
        let func_name = "process_message";
        unsafe {
            if self.lsb == std::ptr::mut_null() {
                return 1;
            }
            let lua = lsb_get_lua(self.lsb);
            if lua == std::ptr::mut_null() {
                return 1;
            }

            let mut r: c_int = 0;
            func_name.with_c_str(|f| { r = lsb_pcall_setup(self.lsb, f); });

            if r != 0 {
                let err = format!("{}() function was not found", func_name);
                err.with_c_str(|e| {lsb_terminate(self.lsb, e);});
                return 1;
            }

            self.msg = Some(msg.clone()); // ideally it should be borrowed but having issues with the lifetime
            if lua_pcall(lua, 0, 1, 0) != 0 {
                let mut len: size_t = 0;
                let err = format!("{}() {}", func_name,  std::str::raw::from_c_str(lua_tolstring(lua, -1, &mut len)));
                err.with_c_str(|e| {lsb_terminate(self.lsb, e);});
                self.msg.take_unwrap();
                return 1;
            }
            self.msg.take_unwrap();

            if lua_isnumber(lua, 1) == 0  {
                let err = format!("{}() must return a single numeric value", func_name);
                err.with_c_str(|e| {lsb_terminate(self.lsb, e);});
                return 1;
            }

            let status = lua_tointeger(lua, 1);
            lua_settop(lua, -2);
            lsb_pcall_teardown(self. lsb);

            return status;
        }
    }

    pub fn timer_event(&mut self, ns: c_longlong) -> c_int {
        let func_name = "timer_event";
        unsafe {
            if self.lsb == std::ptr::mut_null() {
                return 1;
            }
            let lua = lsb_get_lua(self.lsb);
            if lua == std::ptr::mut_null() {
                return 1;
            }

            let mut r: c_int = 0;
            func_name.with_c_str(|f| { r = lsb_pcall_setup(self.lsb, f); });

            if r != 0 {
                let err = format!("{}() function was not found", func_name);
                err.with_c_str(|e| {lsb_terminate(self.lsb, e);});
                return 1;
            }

            lua_pushnumber(lua, ns as f64);
            if lua_pcall(lua, 1, 0, 0) != 0 {
                let mut len: size_t = 0;
                let err = format!("{}() {}", func_name,  std::str::raw::from_c_str(lua_tolstring(lua, -1, &mut len)));
                err.with_c_str(|e| {lsb_terminate(self.lsb, e);});
                return 1;
            }
            lsb_pcall_teardown(self.lsb);
            lua_gc(lua, 2, 0); // todo use enum LUA_GCCOLLECT
            return 0;
        }
    }
}

#[link(name = "luasandbox", kind = "static")]
#[link(name = "cjson", kind = "static")]
#[link(name = "lpeg", kind = "static")]
#[link(name = "lua", kind = "static")]
extern "C" {
fn lsb_create(parent: *const Box<LuaSandbox>,
              lua_file: *const c_char,
              require_path: *const c_char,
              memory_limit: c_uint,
              instruction_limit: c_uint,
              output_limit: c_uint) -> *mut LSB;
fn lsb_init(lsb: *mut LSB, state_file: *const c_char) -> c_int;
fn lsb_add_function(lsb: *mut LSB, func: extern "C" fn(*mut LUA) -> c_int, func_name: *const c_char);
fn lsb_get_error(lsb: *mut LSB) -> *const c_char;
fn lsb_get_lua(lsb: *mut LSB) -> *mut LUA;
fn lsb_get_parent(lsb: *mut LSB) -> *const c_void;
fn lsb_pcall_setup(lsb: *mut LSB, func_name: *const c_char) -> c_int;
fn lsb_pcall_teardown(lsb: *mut LSB);
fn lsb_output_userdata(lsb: *mut LSB, index: c_int, append: c_int) -> *const c_char;
fn lsb_output_protobuf(lsb: *mut LSB, index: c_int, append: c_int) -> c_int;
fn lsb_get_output(lsb: *mut LSB, len: *mut size_t) -> *const c_char;
fn lsb_terminate(lsb: *mut LSB, err: *const c_char);
fn lsb_destroy(lsb: *mut LSB, state_file: *const c_char) -> *mut c_char;

fn lua_error(lua: *mut LUA) -> c_int; // long jumps and never returns
fn lua_type(lua: *mut LUA, index: c_int) -> c_int;
fn lua_pcall(lua: *mut LUA, nargs: c_int, nresults: c_int, errfunc: c_int) -> c_int;
fn lua_isnumber(lua: *mut LUA, index: c_int) -> c_int;
fn lua_tointeger(lua: *mut LUA, index: c_int) -> c_int;
fn lua_tonumber(lua: *mut LUA, index: c_int) -> c_double;
fn lua_tolstring(lua: *mut LUA, index: c_int, len: *mut size_t) -> *const c_char;
fn lua_pushinteger(lua: *mut LUA, ns: c_int);
fn lua_pushnumber(lua: *mut LUA, ns: c_double);
fn lua_pushlstring(lua: *mut LUA, s: *const c_char, len: size_t);
fn lua_pushnil(lua: *mut LUA);
fn lua_gc(lua: *mut LUA, what: c_int, data: c_int) -> c_int;
fn lua_touserdata(lua: *mut LUA, index: c_int) -> *const c_void;
fn lua_gettop(lua: *mut LUA) -> c_int;
fn lua_settop(lua: *mut LUA, index: c_int);
}

extern fn inject_message(lua: *mut LUA) -> c_int {
    unsafe {
        let luserdata = lua_touserdata(lua, -10003); // todo use LUA_GLOBALSINDEX
        if luserdata == std::ptr::null() {
            let err = "inject_message() invalid lightuserdata";
            lua_pushlstring(lua, err.as_ptr() as *const i8, err.len() as size_t);
            return lua_error(lua);
        }
        let top = lua_gettop(lua);
        if top != 1 || lua_type(lua, 1) != 5 { // todo us LUA_TTABLE constant
            let err = "inject_message() takes a single table argument";
            lua_pushlstring(lua, err.as_ptr() as *const i8, err.len() as size_t);
            return lua_error(lua);
        }

        let lsb = luserdata as *mut LSB;
        if lsb_output_protobuf(lsb, 1, 0) != 0 {
            // "inject_message() could not encode protobuf - %s", lsb_get_error(lsb) // todo improve error message
            let err = "inject_message() could not encode protobuf";
            lua_pushlstring(lua, err.as_ptr() as *const i8, err.len() as size_t);
            return lua_error(lua);
        }

        let mut len: size_t = 0;
        let c = lsb_get_output(lsb, &mut len);
        if len != 0 {
            let m = std::slice::raw::buf_as_slice(c as *const u8, len as uint, protobuf::parse_from_bytes::<message::HekaMessage>);
            println!("timestamp:{} fields:{}", m.get_timestamp(), m.get_fields());
        }
        return 0;
    }
}

extern fn read_message(lua: *mut LUA) -> c_int {
    unsafe {
        let luserdata = lua_touserdata(lua, -10003); // todo use LUA_GLOBALSINDEX
        if luserdata == std::ptr::null() {
            let err = "read_message() invalid lightuserdata";
            lua_pushlstring(lua, err.as_ptr() as *const i8, err.len() as size_t);
            return lua_error(lua);
        }

        let lsb = luserdata as *mut LSB;
        let sandbox = lsb_get_parent(lsb) as *mut Box<LuaSandbox>;
        if (*sandbox).msg.is_none() { // doesn't see the message added in process_message
            lua_pushnil(lua);
            return 1;
        }
        let msg = (*sandbox).msg.get_ref();

        let n = lua_gettop(lua);
        if n < 1 || n > 3 {
            let err = "read_message() incorrect number of arguments";
            lua_pushlstring(lua, err.as_ptr() as *const i8, err.len() as size_t);
            return lua_error(lua);
        }

        let mut len: size_t = 0;
        let c = lua_tolstring(lua, 1, &mut len);
        if c == std::ptr::null() {
            let err = "read_message() field argument must be a string";
            lua_pushlstring(lua, err.as_ptr() as *const i8, err.len() as size_t);
            return lua_error(lua);
        }
        let fi = lua_tointeger(lua, 2);
        if fi < 0 {
            let err = "read_message() field index must be positive";
            lua_pushlstring(lua, err.as_ptr() as *const i8, err.len() as size_t);
            return lua_error(lua);
        }
        let ai = lua_tointeger(lua, 3);
        if ai < 0 {
            let err = "read_message() array index must be positive";
            lua_pushlstring(lua, err.as_ptr() as *const i8, err.len() as size_t);
            return lua_error(lua);
        }

        let field = std::string::raw::from_buf(c as *const u8);

        match field.as_slice() {
            "Type" => {
                let s = (*msg).get_field_type();
                lua_pushlstring(lua, s.as_ptr() as *const i8, s.len() as size_t);
            }
            "Logger" => {
                let s = (*msg).get_logger();
                lua_pushlstring(lua, s.as_ptr() as *const i8, s.len() as size_t);
            }
            "Payload" => {
                let s = (*msg).get_payload();
                lua_pushlstring(lua, s.as_ptr() as *const i8, s.len() as size_t);
            }
            "EnvVersion" => {
                let s = (*msg).get_env_version();
                lua_pushlstring(lua, s.as_ptr() as *const i8, s.len() as size_t);
            }
            "Hostname" => {
                let s = (*msg).get_hostname();
                lua_pushlstring(lua, s.as_ptr() as *const i8, s.len() as size_t);
            }
            "Uuid" => {
                let s = (*msg).get_uuid();
                lua_pushlstring(lua, s.as_ptr() as *const i8, s.len() as size_t);
            }
            "Timestamp" => {
                lua_pushnumber(lua, (*msg).get_timestamp() as f64);
            }
            "Severity" => {
                lua_pushinteger(lua, (*msg).get_severity());
            }
            "Pid" => {
                lua_pushinteger(lua, (*msg).get_pid());
            }
            "raw" => {
                let s = (*msg).get_payload();
                lua_pushlstring(lua, s.as_ptr() as *const i8, s.len() as size_t);
            }
            _ => {
                lua_pushnil(lua);
            }
        }
        return 1;
    }
}
