extern crate protobuf; // depend on rust-protobuf runtime
extern crate std;

use libc::{c_int, c_uint, c_char, size_t, c_longlong, c_void, c_double};
use std::any::{Any, AnyRefExt};
use std::collections::HashMap;
use std::c_str::CString;

use message;

//trait Sandbox {
//  fn new(source_file: &[u8], include_path: &[u8], memory_limit: c_uint, instruction_limit: c_uint, output_limit: c_uint) -> Self;
//  fn init(&mut self, state_file: &[u8]) -> int;
//  fn destroy(&mut self, state_file: &[u8]) -> String
//  fn last_error(&mut self) -> Sting;
//  fn fn process_message(&mut self, msg: message::HekaMessage) -> (c_int, message::HekaMessage);
//  fn timer_event(&mut self, ns: i64) -> int;
//  fn usage(&mut self, utype: lsb_usage_type, ustat: lsb_usage_stat) -> c_uint;
//  fn state(&mut self) -> lsb_state;
//}

#[repr(C)]
pub enum lsb_state {
  STATE_UNKNOWN      = 0,
  STATE_RUNNING      = 1,
  STATE_TERMINATED   = 2
}

#[repr(C)]
pub enum lsb_usage_type {
  TYPE_MEMORY       = 0,
  TYPE_INSTRUCTION  = 1,
  TYPE_OUTPUT       = 2,
  TYPE_MAX
}

#[repr(C)]
pub enum lsb_usage_stat {
  STAT_LIMIT    = 0,
  STAT_CURRENT  = 1,
  STAT_MAXIMUM  = 2,
  STAT_MAX
}

struct SandboxConfig {
    config: HashMap<String, Box<Any>>
}

pub enum LSB {}
pub enum LUA {}
pub struct LuaSandbox {
    msg: std::option::Option<message::HekaMessage>,
    lsb: *mut LSB,
    config: SandboxConfig,
    field_iterator: uint
}

impl LuaSandbox {
    pub fn new(lua_file: &[u8],
               require_path: &[u8],
               memory_limit: c_uint,
               instruction_limit: c_uint,
               output_limit: c_uint) -> Box<LuaSandbox> {
        unsafe {
            let cfg = SandboxConfig {
                config: HashMap::new()
            };

            let mut s = box LuaSandbox{
                msg: None,
                lsb: std::ptr::mut_null(),
                config: cfg,
                field_iterator: 0
            };
            // Convert our owned box into an unsafe pointer, making
            // sure that we're passing a pointer into the heap to
            // lsb_create. The way this stores a mutable unsafe
            // pointer to somebody else's owned box is pretty sketchy
            // and we're going to have to be careful not to invoke
            // undefined behavior. This can probably be restructured
            // so that lsb owns whatever data it needs.
            let unsafe_s = &mut *s as *mut LuaSandbox as *mut c_void;
            lua_file.with_c_str(|lf| {
                require_path.with_c_str(|rp| {
                s.lsb = lsb_create(unsafe_s, lf, rp, memory_limit, instruction_limit, output_limit);
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
            "inject_payload".with_c_str(|f| {lsb_add_function(self.lsb, inject_payload, f);});
            "read_message".with_c_str(|f| {lsb_add_function(self.lsb, read_message, f);});
            "read_config".with_c_str(|f| {lsb_add_function(self.lsb, read_config, f);});
            "read_next_field".with_c_str(|f| {lsb_add_function(self.lsb, read_next_field, f);});

            let mut r: c_int = 0;
            state_file.with_c_str(|sf| {r = lsb_init(self.lsb, sf);}) ;
            if r != 0 {
                return r;
            }

            // todo rename output to add_to_payload
            return 0;
        }
    }

    pub fn destroy(&mut self, state_file: &[u8]) -> String {
        unsafe {
            if self.lsb == std::ptr::mut_null() {
                return String::new();
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
                return String::from_str("creation failed");
            }
            let c = lsb_get_error(self.lsb);
            if c != std::ptr::null() {
                return std::string::raw::from_buf(c as *const u8);
            } else {
                return String::new();
            }
        }
    }

    pub fn usage(&mut self, utype: lsb_usage_type, ustat: lsb_usage_stat) -> c_uint {
        unsafe {
            if self.lsb == std::ptr::mut_null() {
                return 0;
            }
            return lsb_usage(self.lsb, utype, ustat);
        }
    }

    pub fn state(&mut self) -> lsb_state {
        unsafe {
            if self.lsb == std::ptr::mut_null() {
                return STATE_UNKNOWN;
            }
            return lsb_get_state(self.lsb);
        }
    }

    pub fn process_message(&mut self, msg: message::HekaMessage) -> (c_int, message::HekaMessage) {
        let func_name = "process_message";
        unsafe {
            if self.lsb == std::ptr::mut_null() {
                return (1, msg);
            }
            let lua = lsb_get_lua(self.lsb);
            if lua == std::ptr::mut_null() {
                return (1, msg);
            }

            let mut r: c_int = 0;
            func_name.with_c_str(|f| { r = lsb_pcall_setup(self.lsb, f); });

            if r != 0 {
                let err = format!("{}() function was not found", func_name);
                err.with_c_str(|e| {lsb_terminate(self.lsb, e);});
                return (1, msg);
            }

            assert!(self.msg.is_none());
            self.msg = Some(msg);
            self.field_iterator = 0;
            if lua_pcall(lua, 0, 1, 0) != 0 {
                let mut len: size_t = 0;
                let c = lua_tolstring(lua, -1, &mut len);
                let err = format!("{}() {}", func_name, std::string::raw::from_buf_len(c as *const u8, len as uint));
                err.with_c_str(|e| {lsb_terminate(self.lsb, e);});
                return (1, self.msg.take_unwrap());
            }

            if lua_isnumber(lua, 1) == 0  {
                let err = format!("{}() must return a single numeric value", func_name);
                err.with_c_str(|e| {lsb_terminate(self.lsb, e);});
                return (1, self.msg.take_unwrap());
            }

            let status = lua_tointeger(lua, 1);
            lua_settop(lua, -2);
            lsb_pcall_teardown(self. lsb);

            return (status, self.msg.take_unwrap());
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
                let c = lua_tolstring(lua, -1, &mut len);
                let err = format!("{}() {}", func_name,  std::string::raw::from_buf_len(c as *const u8, len as uint));
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
fn lsb_create(parent: *mut c_void,
              lua_file: *const c_char,
              require_path: *const c_char,
              memory_limit: c_uint,
              instruction_limit: c_uint,
              output_limit: c_uint) -> *mut LSB;
fn lsb_init(lsb: *mut LSB, state_file: *const c_char) -> c_int;
fn lsb_add_function(lsb: *mut LSB, func: extern "C" fn(*mut LUA) -> c_int, func_name: *const c_char);
fn lsb_get_error(lsb: *mut LSB) -> *const c_char;
fn lsb_get_lua(lsb: *mut LSB) -> *mut LUA;
fn lsb_get_parent(lsb: *mut LSB) -> *mut c_void;
fn lsb_pcall_setup(lsb: *mut LSB, func_name: *const c_char) -> c_int;
fn lsb_pcall_teardown(lsb: *mut LSB);
fn lsb_output_userdata(lsb: *mut LSB, index: c_int, append: c_int) -> *const c_char;
fn lsb_output_protobuf(lsb: *mut LSB, index: c_int, append: c_int) -> c_int;
fn lsb_output(lsb: *mut LSB, start: c_int, end: c_int, append: c_int);
fn lsb_get_output(lsb: *mut LSB, len: *mut size_t) -> *const c_char;
fn lsb_terminate(lsb: *mut LSB, err: *const c_char);
fn lsb_destroy(lsb: *mut LSB, state_file: *const c_char) -> *mut c_char;
fn lsb_usage(lsb: *mut LSB, utype: lsb_usage_type, ustat: lsb_usage_stat) -> c_uint;
fn lsb_get_state(lsb: *mut LSB) -> lsb_state;

fn lua_error(lua: *mut LUA) -> c_int; // long jumps and never returns
fn lua_type(lua: *mut LUA, index: c_int) -> c_int;
fn lua_pcall(lua: *mut LUA, nargs: c_int, nresults: c_int, errfunc: c_int) -> c_int;
fn lua_isnumber(lua: *mut LUA, index: c_int) -> c_int;
fn lua_tointeger(lua: *mut LUA, index: c_int) -> c_int;
fn lua_tonumber(lua: *mut LUA, index: c_int) -> c_double;
fn lua_tolstring(lua: *mut LUA, index: c_int, len: *mut size_t) -> *const c_char;
fn lua_pushinteger(lua: *mut LUA, ns: c_int);
fn lua_pushnumber(lua: *mut LUA, ns: c_double);
fn lua_pushboolean(lua: *mut LUA, b: c_int);
fn lua_pushlstring(lua: *mut LUA, s: *const c_char, len: size_t);
fn lua_pushnil(lua: *mut LUA);
fn lua_gc(lua: *mut LUA, what: c_int, data: c_int) -> c_int;
fn lua_touserdata(lua: *mut LUA, index: c_int) -> *const c_void;
fn lua_gettop(lua: *mut LUA) -> c_int;
fn lua_settop(lua: *mut LUA, index: c_int);
fn luaL_checklstring(lua: *mut LUA, index: c_int, len: *mut size_t) -> *const c_char;
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
            let err = format!("inject_message() could not encode protobuf: {}", lsb_get_error(lsb));
            lua_pushlstring(lua, err.as_slice().as_ptr() as *const i8, err.len() as size_t);
            return lua_error(lua);
        }

        let mut len: size_t = 0;
        let c = lsb_get_output(lsb, &mut len);
        if len != 0 {
            // todo hand it back to a Rust callback
            let m = std::slice::raw::buf_as_slice(c as *const u8, len as uint, protobuf::parse_from_bytes::<message::HekaMessage>);
            println!("timestamp:{} fields:{}", m.get_timestamp(), m.get_fields());
        }
        return 0;
    }
}

extern fn inject_payload(lua: *mut LUA) -> c_int {
    unsafe {
        let luserdata = lua_touserdata(lua, -10003); // todo use LUA_GLOBALSINDEX
        if luserdata == std::ptr::null() {
            let err = "inject_payload() invalid lightuserdata";
            lua_pushlstring(lua, err.as_ptr() as *const i8, err.len() as size_t);
            return lua_error(lua);
        }
        let lsb = luserdata as *mut LSB;
        let mut len: size_t = 0;
        let mut typ = String::from_str("txt");
        let mut name = String::new();
        let top = lua_gettop(lua);
        if top > 0 {
            let c = luaL_checklstring(lua, 1, &mut len);
            if len > 0 {
                typ = std::string::raw::from_buf_len(c as *const u8, len as uint);
            }
        }
        if top > 1 {
            let c = luaL_checklstring(lua, 2, &mut len);
            name = std::string::raw::from_buf_len(c as *const u8, len as uint);
        }
        if top > 2 {
            lsb_output(lsb, 3, top, 1);
        }
        let c = lsb_get_output(lsb, &mut len);
        if len != 0 {
            // todo hand it back to a Rust callback
            println!("name:'{}' type:'{}' output:'{}'", name, typ, std::str::raw::from_buf_len(c as *const u8, len as uint));
        }
        return 0;
    }
}

fn find_field<'a>(msg: &'a message::HekaMessage, name: &str, fi: uint) -> Option<&'a message::Field> {
    let mut cnt = 0u;
    for value in msg.get_fields().iter() {
        if name == value.get_name() {
            if cnt == fi {
                return Some(value);
            }
            cnt += 1;
        }
    };
    None
}

fn get_field_name<'a>(key: &'a str) -> Option<&'a str> {
    let l = key.len();
    if l > 0 && key.char_at(l-1) == ']' {
        if key.starts_with("Fields[") {
            return Some(key.slice(7, l-1));
        }
    }
    None
}

fn push_field(lua: *mut LUA, field: &message::Field, ai: uint) -> uint {
    match field.get_value_type() {
        message::Field_STRING => {
            let v = field.get_value_string();
            if ai < v.len() {
                let ref s = v[ai].as_slice();
                unsafe {
                    lua_pushlstring(lua, s.as_ptr() as *const i8, s.len() as size_t);
                }
                return v.len();
            }
        }
        message::Field_BYTES => {
            let v = field.get_value_bytes();
            if ai < v.len() {
                let ref s = v[ai];
                unsafe {
                    lua_pushlstring(lua, s.as_ptr() as *const i8, s.len() as size_t);
                }
                return v.len();
            }
        }
        message::Field_INTEGER => {
            let v = field.get_value_integer();
            if ai < v.len() {
                unsafe {
                    lua_pushnumber(lua,  v[ai] as f64);
                }
                return v.len();
            }
        }
        message::Field_DOUBLE => {
            let v = field.get_value_double();
            if ai < v.len() {
                unsafe {
                    lua_pushnumber(lua,  v[ai]);
                }
                return v.len();
            }
        }
        message::Field_BOOL => {
            let v = field.get_value_bool();
            if ai < v.len() {
                unsafe {
                    lua_pushboolean(lua,  v[ai] as c_int);
                }
                return v.len();
            }
        }
    }

    unsafe {
        lua_pushnil(lua);
    }
    return 0;
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
        let sandbox = lsb_get_parent(lsb) as *mut LuaSandbox;
        assert!(!sandbox.is_null());
        if (*sandbox).msg.is_none() { // read attempt outside process_message is non-fatal
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
                let s = msg.get_field_type();
                lua_pushlstring(lua, s.as_ptr() as *const i8, s.len() as size_t);
            }
            "Logger" => {
                let s = msg.get_logger();
                lua_pushlstring(lua, s.as_ptr() as *const i8, s.len() as size_t);
            }
            "Payload" => {
                let s = msg.get_payload();
                lua_pushlstring(lua, s.as_ptr() as *const i8, s.len() as size_t);
            }
            "EnvVersion" => {
                let s = msg.get_env_version();
                lua_pushlstring(lua, s.as_ptr() as *const i8, s.len() as size_t);
            }
            "Hostname" => {
                let s = msg.get_hostname();
                lua_pushlstring(lua, s.as_ptr() as *const i8, s.len() as size_t);
            }
//            "Uuid" => {
//                let u = Uuid::from_bytes(msg.get_uuid());
//                let s = u.get_ref().to_simple_str();
//                lua_pushlstring(lua, s.as_slice().as_ptr() as *const i8, s.len() as size_t);
//            }
            "Timestamp" => {
                lua_pushnumber(lua, msg.get_timestamp() as f64);
            }
            "Severity" => {
                lua_pushinteger(lua, msg.get_severity());
            }
            "Pid" => {
                lua_pushinteger(lua, msg.get_pid());
            }
            "raw" => {
                let s = msg.get_payload();
                lua_pushlstring(lua, s.as_ptr() as *const i8, s.len() as size_t);
            }
            _ => {
                match get_field_name(field.as_slice())
                {
                    Some(name) => {
                        match find_field(msg, name, fi as uint)
                        {
                            Some(f) => {push_field(lua, f, ai as uint);}
                            None => {lua_pushnil(lua);}
                        }
                    }
                    None => {lua_pushnil(lua);}
                }
            }
        }
    }
    return 1;
}

extern fn read_next_field(lua: *mut LUA) -> c_int {
    unsafe {
        let luserdata = lua_touserdata(lua, -10003); // todo use LUA_GLOBALSINDEX
        if luserdata == std::ptr::null() {
            let err = "read_next_field() invalid lightuserdata";
            lua_pushlstring(lua, err.as_ptr() as *const i8, err.len() as size_t);
            return lua_error(lua);
        }

        if lua_gettop(lua) !=0 {
            let err = "read_next_field() takes no arguments";
            lua_pushlstring(lua, err.as_ptr() as *const i8, err.len() as size_t);
            return lua_error(lua);
        }

        let lsb = luserdata as *mut LSB;
        let sandbox = lsb_get_parent(lsb) as *mut LuaSandbox;
        assert!(!sandbox.is_null());

        if (*sandbox).msg.is_none()  // read attempt outside process_message is non-fatal
        || (*sandbox).field_iterator >= (*sandbox).msg.get_ref().get_fields().len() { // finished iterating
            lua_pushnil(lua);
            lua_pushnil(lua);
            lua_pushnil(lua);
            lua_pushnil(lua);
            lua_pushnil(lua);
            return 5;
        }

        let msg = (*sandbox).msg.get_ref();
        let ref field = msg.get_fields()[(*sandbox).field_iterator];
        (*sandbox).field_iterator += 1;
        lua_pushinteger(lua, field.get_value_type() as c_int);
        lua_pushlstring(lua, field.get_name().as_ptr() as *const i8, field.get_name().len() as size_t);
        let count = push_field(lua, field, 0);
        lua_pushlstring(lua, field.get_representation().as_ptr() as *const i8, field.get_representation().len() as size_t);
        lua_pushinteger(lua, count as c_int);
    }
    return 5;
}

extern fn read_config(lua: *mut LUA) -> c_int {
    unsafe {
        let luserdata = lua_touserdata(lua, -10003); // todo use LUA_GLOBALSINDEX
        if luserdata == std::ptr::null() {
            let err = "read_config() invalid lightuserdata";
            lua_pushlstring(lua, err.as_ptr() as *const i8, err.len() as size_t);
            return lua_error(lua);
        }

        let lsb = luserdata as *mut LSB;
        let sandbox = lsb_get_parent(lsb) as *mut LuaSandbox;
        assert!(!sandbox.is_null());
        let sandbox: &mut LuaSandbox = &mut (*sandbox); // Get a Rust pointer

        if lua_gettop(lua) != 1 {
            let err = "read_config() must have a single argument";
            lua_pushlstring(lua, err.as_ptr() as *const i8, err.len() as size_t);
            return lua_error(lua);
        }

        // Get the config key as a Rust string
        let mut len: size_t = 0;
        let name: *const c_char = luaL_checklstring(lua, 1, &mut len);
        if name.is_null() {
            let err = "read_config() argument must be a string";
            lua_pushlstring(lua, err.as_ptr() as *const i8, err.len() as size_t);
            return lua_error(lua);
        }
        let name = CString::new(name, false);
        let name = name.as_str().unwrap(); // Unlikely to fail

        let ref config_map = sandbox.config.config;
        match config_map.find_equiv(&name) {
            Some(val) if val.is::<String>() => {
                let s: &String = val.downcast_ref::<String>().unwrap();
                s.with_c_str(|cstr| lua_pushlstring(lua, cstr, s.len() as size_t));
            }
            Some(val) if val.is::<f64>() => {
                lua_pushnumber(lua, *val.downcast_ref::<f64>().unwrap() as c_double)
            }
            Some(val) if val.is::<i64>() => {
                lua_pushnumber(lua, *val.downcast_ref::<i64>().unwrap() as c_double)
            }
            Some(val) if val.is::<bool>() => {
                lua_pushboolean(lua, *val.downcast_ref::<bool>().unwrap() as c_int)
            }
            Some(_) | None => {
                lua_pushnil(lua);
            }
        }
    }
    return 1;
}

#[cfg(test)]
mod test {
    use std::any::Any;
    use sandbox;
    use message;

    #[test]
    fn creation_failed() {
        let mut sb = sandbox::LuaSandbox::new("".as_bytes(), "".as_bytes(), 32767, 1000, 9*1024*1024);
        // make sure all the functions properly handle the bad state
        assert!(sb.last_error().as_slice() == "creation failed");
        assert!(sb.state() as int == sandbox::STATE_UNKNOWN as int);
        assert!(-1 == sb.init("".as_bytes()));
        assert!(0 == sb.usage(sandbox::TYPE_MEMORY, sandbox::STAT_CURRENT));
        let mut m = Some(message::HekaMessage::new());
        let (mut rc, mm) = sb.process_message(m.take_unwrap());
        m = Some(mm);
        assert!(1 == rc);
        rc = sb.timer_event(0);
        assert!(1 == rc);
        assert!(sb.destroy("".as_bytes()).is_empty());
    }

    #[test]
    fn init_failed() {
        let mut sb = sandbox::LuaSandbox::new("../test/not_found.lua".as_bytes(), "".as_bytes(), 32767, 1000, 1024);
        assert!(sb.last_error().is_empty());
        assert!(0 != sb.init("".as_bytes()));
        assert!(sb.state() as int == sandbox::STATE_TERMINATED as int);
        assert!(sb.last_error().as_slice() == "cannot open ../test/not_found.lua: No such file or directory");
        assert!(sb.destroy("".as_bytes()).is_empty());
    }

    #[test]
    fn init() {
        let mut sb = sandbox::LuaSandbox::new("../test/hello_world.lua".as_bytes(), "".as_bytes(), 32767, 1000, 1024);
        assert!(sb.last_error().is_empty());
        assert!(1000 == sb.usage(sandbox::TYPE_INSTRUCTION, sandbox::STAT_LIMIT));
        assert!(1024 == sb.usage(sandbox::TYPE_OUTPUT, sandbox::STAT_LIMIT));
        assert!(0 == sb.init("".as_bytes()));
        assert!(sb.state() as int == sandbox::STATE_RUNNING as int); // todo ask Brian how to test without the conversion
        assert!(0 < sb.usage(sandbox::TYPE_MEMORY, sandbox::STAT_CURRENT));
        assert!(0 < sb.usage(sandbox::TYPE_MEMORY, sandbox::STAT_MAXIMUM));
        assert!(0 < sb.usage(sandbox::TYPE_MEMORY, sandbox::STAT_LIMIT));
        assert!(0 < sb.usage(sandbox::TYPE_INSTRUCTION, sandbox::STAT_CURRENT));
        assert!(0 < sb.usage(sandbox::TYPE_INSTRUCTION, sandbox::STAT_MAXIMUM));
        assert!(0 < sb.usage(sandbox::TYPE_OUTPUT, sandbox::STAT_CURRENT));
        assert!(0 < sb.usage(sandbox::TYPE_OUTPUT, sandbox::STAT_MAXIMUM));
        assert!(sb.destroy("".as_bytes()).is_empty());
    }

    #[test]
    fn process_message_missing() {
        let mut sb = sandbox::LuaSandbox::new("../test/hello_world.lua".as_bytes(), "".as_bytes(), 32767, 1000, 1024);
        assert!(sb.last_error().is_empty());
        assert!(0 == sb.init("".as_bytes()));
        let mut m = Some(message::HekaMessage::new());
        let (rc, mm) = sb.process_message(m.take_unwrap());
        m = Some(mm);
        assert!(rc != 0);
        assert!(sb.state() as int == sandbox::STATE_TERMINATED as int);
        assert!(sb.last_error().as_slice() == "process_message() function was not found");
        assert!(sb.destroy("".as_bytes()).is_empty());
    }

    #[test]
    fn timer_event_missing() {
        let mut sb = sandbox::LuaSandbox::new("../test/hello_world.lua".as_bytes(), "".as_bytes(), 32767, 1000, 1024);
        assert!(sb.last_error().is_empty());
        assert!(0 == sb.init("".as_bytes()));
        let rc = sb.timer_event(0);
        assert!(rc != 0);
        assert!(sb.state() as int == sandbox::STATE_TERMINATED as int);
        assert!(sb.last_error().as_slice() == "timer_event() function was not found");
        assert!(sb.destroy("".as_bytes()).is_empty());
    }

    #[test]
    fn read_message() {
        let mut sb = sandbox::LuaSandbox::new("../test/read_message.lua".as_bytes(), "".as_bytes(), 64*1024, 0, 0);
        assert!(sb.last_error().is_empty());
        assert!(0 == sb.init("".as_bytes()));
        let mut m = Some(message::HekaMessage::new());
        m.get_mut_ref().set_field_type(String::from_str("type"));
        m.get_mut_ref().set_logger(String::from_str("logger"));
        m.get_mut_ref().set_payload(String::from_str("payload"));
        m.get_mut_ref().set_env_version(String::from_str("envversion"));
        m.get_mut_ref().set_hostname(String::from_str("hostname"));
//        let (u, _) = Uuid::parse_string("f47ac10b-58cc-4372-a567-0e02b2c3d479");
//        m.get_mut_ref().set_uuid(u.as_bytes());
        m.get_mut_ref().set_timestamp(999);
        m.get_mut_ref().set_severity(4);
        m.get_mut_ref().set_pid(23);
        let mut f = message::Field::new();
        f.set_name("test".into_string());
        f.set_value_type(message::Field_STRING);
        f.add_value_string("foo".into_string());
        f.add_value_string("bar".into_string());
        m.get_mut_ref().add_fields(f);
        let mut f1 = message::Field::new();
        f1.set_name("widget".into_string());
        f1.set_value_type(message::Field_INTEGER);
        f1.add_value_integer(222);
        m.get_mut_ref().add_fields(f1);
        let mut f2 = message::Field::new();
        f2.set_name("test".into_string());
        f2.set_value_type(message::Field_STRING);
        f2.add_value_string("foo1".into_string());
        f2.add_value_string("bar1".into_string());
        m.get_mut_ref().add_fields(f2);
        let (rc, mm) = sb.process_message(m.take_unwrap());
        m = Some(mm);
        assert!(rc == 0, "{}", sb.last_error());
        assert!(sb.destroy("".as_bytes()).is_empty());
    }

    #[test]
    fn read_message_not_available() {
        let mut sb = sandbox::LuaSandbox::new("../test/read_message.lua".as_bytes(), "".as_bytes(), 64*1024, 0, 0);
        assert!(sb.last_error().is_empty());
        assert!(0 == sb.init("".as_bytes()));
        let rc = sb.timer_event(0);
        assert!(rc == 0, "{}", sb.last_error());
        assert!(sb.destroy("".as_bytes()).is_empty());
    }

    #[test]
    fn read_message_error() {
        let err = vec!["process_message() read_message() incorrect number of arguments",
         "process_message() read_message() incorrect number of arguments",
         "process_message() read_message() field argument must be a string",
         "process_message() read_message() field index must be positive",
         "process_message() read_message() array index must be positive"];
        let mut idx = 0;
        let mut m = Some(message::HekaMessage::new());
        for e in err.iter() {
            let mut sb = sandbox::LuaSandbox::new("../test/read_message_error.lua".as_bytes(), "".as_bytes(), 64*1024, 0, 0);
            assert!(sb.last_error().is_empty());
            assert!(0 == sb.init("".as_bytes()));
            m.get_mut_ref().set_pid(idx);
            let (rc, mm) = sb.process_message(m.take_unwrap());
            m = Some(mm);
            assert!(rc == 1);
            assert!(sb.last_error().as_slice() == *e, "test: {} expected: {} received: {}", idx, *e, sb.last_error());
            idx += 1;
            assert!(sb.destroy("".as_bytes()).is_empty());
        }
    }

    #[test]
    fn read_config() {
        let mut sb = sandbox::LuaSandbox::new("../test/read_config.lua".as_bytes(), "".as_bytes(), 64*1024, 1000, 1024);
        assert!(sb.last_error().is_empty());

        sb.config.config.insert("string".to_string(), box "widget".to_string() as Box<Any>);
        sb.config.config.insert("int64".to_string(), box () (99 as i64) as Box<Any>);
        sb.config.config.insert("double".to_string(), box () (99.123 as f64) as Box<Any>);
        sb.config.config.insert("bool".to_string(), box true as Box<Any>);
        assert!(0 == sb.init("".as_bytes()), "{}", sb.last_error());
        assert!(sb.destroy("".as_bytes()).is_empty());
    }

    #[test]
    fn read_next_field() {
        let mut sb = sandbox::LuaSandbox::new("../test/read_next_field.lua".as_bytes(), "".as_bytes(), 64*1024, 0, 0);
        assert!(sb.last_error().is_empty());
        assert!(0 == sb.init("".as_bytes()));
        let mut m = Some(message::HekaMessage::new());

        let mut f = message::Field::new();
        f.set_name("foo".into_string());
        f.set_representation("test".into_string());
        f.set_value_type(message::Field_STRING);
        f.add_value_string("bar".into_string());
        m.get_mut_ref().add_fields(f);


        let mut f1 = message::Field::new();
        f1.set_name("bytes".into_string());
        f1.set_value_type(message::Field_BYTES);
        f1.add_value_bytes(vec!['d' as u8, 'a' as u8, 't' as u8, 'a' as u8]);
        m.get_mut_ref().add_fields(f1);

        let mut f2 = message::Field::new();
        f2.set_name("int".into_string());
        f2.set_value_type(message::Field_INTEGER);
        f2.add_value_integer(999);
        f2.add_value_integer(1000);
        m.get_mut_ref().add_fields(f2);

        let mut f3 = message::Field::new();
        f3.set_name("double".into_string());
        f3.set_value_type(message::Field_DOUBLE);
        f3.add_value_double(99.9);
        m.get_mut_ref().add_fields(f3);

        let mut f4 = message::Field::new();
        f4.set_name("bool".into_string());
        f4.set_value_type(message::Field_BOOL);
        f4.add_value_bool(true);
        m.get_mut_ref().add_fields(f4);

        let mut f5 = message::Field::new();
        f5.set_name("foo".into_string());
        f5.set_value_type(message::Field_STRING);
        f5.add_value_string("alternate".into_string());
        m.get_mut_ref().add_fields(f5);

        let mut f6 = message::Field::new();
        f6.set_name("false".into_string());
        f6.set_value_type(message::Field_BOOL);
        f6.add_value_bool(false);
        m.get_mut_ref().add_fields(f6);

        for n in range(0u, 2) { // make sure the iterator is reset between messages
            let (rc, mm) = sb.process_message(m.take_unwrap());
            m = Some(mm);
            assert!(rc == 0, "{}", sb.last_error());
        }

        assert!(sb.destroy("".as_bytes()).is_empty());
    }

    #[test]
    fn read_next_field_error() {
        let mut sb = sandbox::LuaSandbox::new("../test/read_next_field.lua".as_bytes(), "".as_bytes(), 64*1024, 0, 0);
        assert!(sb.last_error().is_empty());
        assert!(0 == sb.init("".as_bytes()));
        let rc = sb.timer_event(0);
        assert!(1 == rc);
        assert!(sb.last_error().as_slice() == "timer_event() read_next_field() takes no arguments", "received: {}", sb.last_error());
        assert!(sb.destroy("".as_bytes()).is_empty());
    }
}
