use std;
use std::any::{Any, AnyRefExt};
use std::collections::HashMap;
use std::c_str::CString;
use libc::{c_int, c_uint, c_char, size_t, c_longlong, c_void, c_double};
use protobuf;
use super::message::pb;

//trait Sandbox {
//  fn new(source_file: &[u8], include_path: &[u8], memory_limit: c_uint, instruction_limit: c_uint, output_limit: c_uint) -> Self;
//  fn init(&mut self, state_file: &[u8]) -> int;
//  fn destroy(&mut self, state_file: &[u8]) -> String
//  fn last_error(&mut self) -> Sting;
//  fn fn process_message(&mut self, msg: pb::HekaMessage) -> (c_int, pb::HekaMessage);
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

#[repr(C)]
pub enum lua_pseudo_index {
  LUA_GLOBALSINDEX = -10002,
  LUA_UPVALUEINDEX = -10003
}

struct SandboxConfig {
    config: HashMap<String, Box<Any>>
}

pub enum LSB {}
pub enum LUA {}
pub struct LuaSandbox {
    msg: std::option::Option<pb::HekaMessage>,
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
            // todo should only be made available to decoders/encoders
            // if we are going to share an immutable view of the message we need to implement
            // copy on write semantics (like we do in the Go version)
            "write_message".with_c_str(|f| {lsb_add_function(self.lsb, write_message, f);});

            let mut r: c_int = 0;
            state_file.with_c_str(|sf| {r = lsb_init(self.lsb, sf);}) ;
            if r != 0 {
                return r;
            }

            // rename output to add_to_payload
            let lua = lsb_get_lua(self.lsb);
            let output = "output";
            output.with_c_str(|f| {lua_getfield(lua, LUA_GLOBALSINDEX as i32, f);});
            "add_to_payload".with_c_str(|f| {lua_setfield(lua, LUA_GLOBALSINDEX as i32, f);});
            lua_pushnil(lua);
            output.with_c_str(|f| {lua_setfield(lua, LUA_GLOBALSINDEX as i32, f);});
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

    pub fn process_message(&mut self, msg: pb::HekaMessage) -> (c_int, pb::HekaMessage) {
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
                let c = lua_tolstring(lua, -1, std::ptr::mut_null());
                let err = format!("{}() {}", func_name, std::string::raw::from_buf(c as *const u8));
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
                let c = lua_tolstring(lua, -1, std::ptr::mut_null());
                let err = format!("{}() {}", func_name,  std::string::raw::from_buf(c as *const u8));
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
fn lua_toboolean(lua: *mut LUA, index: c_int) -> c_int;
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
fn lua_getfield(lua: *mut LUA, index: c_int, k: *const c_char);
fn lua_setfield(lua: *mut LUA, index: c_int, k: *const c_char);

fn luaL_checklstring(lua: *mut LUA, index: c_int, len: *mut size_t) -> *const c_char;
fn luaL_optlstring(lua: *mut LUA, index: c_int, d: *const c_char, len: *mut size_t) -> *const c_char;
fn luaL_optinteger(lua: *mut LUA, narg: c_int, d: c_int) -> c_int;
fn luaL_argerror(lua: *mut LUA, narg: c_int, msg: *const c_char);
}

extern fn inject_message(lua: *mut LUA) -> c_int {
    unsafe {
        argcheck(lua, lua_gettop(lua) == 1 && lua_type(lua, 1) == 5, 1, "takes a single table argument");
        let luserdata = lua_touserdata(lua, LUA_UPVALUEINDEX as i32);
        argcheck(lua, luserdata != std::ptr::null(), 0, "invalid lightuserdata");

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
            let m = std::slice::raw::buf_as_slice(c as *const u8, len as uint, protobuf::parse_from_bytes::<pb::HekaMessage>);
            println!("timestamp:{} fields:{}", m.get_timestamp(), m.get_fields());
        }
        return 0;
    }
}

extern fn inject_payload(lua: *mut LUA) -> c_int {
    unsafe {
        let luserdata = lua_touserdata(lua, LUA_UPVALUEINDEX as i32);
        argcheck(lua, luserdata != std::ptr::null(), 0, "invalid lightuserdata");

        let lsb = luserdata as *mut LSB;
        let mut len: size_t = 0;
        let mut typ = String::from_str("txt");
        let mut name = String::new();
        let top = lua_gettop(lua);
        if top > 0 {
            let c = luaL_checklstring(lua, 1, &mut len);
            if len > 0 {
                typ = std::string::raw::from_buf(c as *const u8);
            }
        }
        if top > 1 {
            let c = luaL_checklstring(lua, 2, std::ptr::mut_null());
            name = std::string::raw::from_buf(c as *const u8);
        }
        if top > 2 {
            lsb_output(lsb, 3, top, 1);
        }
        let c = lsb_get_output(lsb, &mut len);
        if len != 0 {
            // todo hand it back to a Rust callback
            println!("name:'{}' type:'{}' output:'{}'", name, typ, std::str::raw::from_buf_len(c as *const u8, len as uint)); // allow embedded nulls
        }
        return 0;
    }
}

fn argcheck(lua: *mut LUA, cond: bool, narg: c_int, msg: &str) {
    if !cond {
        unsafe {
            lua_pushlstring(lua, msg.as_ptr() as *const i8, msg.len() as size_t); // create a properly terminated NULL string
            luaL_argerror(lua, narg, lua_tolstring(lua, -1, std::ptr::mut_null())); // long jumps
        }
    }
}


fn find_field<'a>(msg: &'a pb::HekaMessage, name: &str, fi: uint) -> Option<&'a pb::Field> {
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

fn find_field_mut<'a>(msg: &'a mut pb::HekaMessage, name: &str, fi: uint) -> (Option<&'a mut pb::Field>, uint) {
    let mut cnt = 0u;
    for value in msg.mut_fields().mut_iter() {
        if name == value.get_name() {
            if cnt == fi {
                return (Some(value), cnt);
            }
            cnt += 1;
        }
    };
    (None, cnt)
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

fn push_field(lua: *mut LUA, field: &pb::Field, ai: uint) -> uint {
    match field.get_value_type() {
        pb::Field_STRING => {
            let v = field.get_value_string();
            if ai < v.len() {
                let ref s = v[ai].as_slice();
                unsafe {
                    lua_pushlstring(lua, s.as_ptr() as *const i8, s.len() as size_t);
                }
                return v.len();
            }
        }
        pb::Field_BYTES => {
            let v = field.get_value_bytes();
            if ai < v.len() {
                let ref s = v[ai];
                unsafe {
                    lua_pushlstring(lua, s.as_ptr() as *const i8, s.len() as size_t);
                }
                return v.len();
            }
        }
        pb::Field_INTEGER => {
            let v = field.get_value_integer();
            if ai < v.len() {
                unsafe {
                    lua_pushnumber(lua,  v[ai] as f64);
                }
                return v.len();
            }
        }
        pb::Field_DOUBLE => {
            let v = field.get_value_double();
            if ai < v.len() {
                unsafe {
                    lua_pushnumber(lua,  v[ai]);
                }
                return v.len();
            }
        }
        pb::Field_BOOL => {
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
        let n = lua_gettop(lua);
        argcheck(lua, n > 0 && n < 4, 0, "incorrect number of arguments");
        let f: *const c_char = luaL_checklstring(lua, 1, std::ptr::mut_null());
        let fi = luaL_optinteger(lua, 2, 0);
        argcheck(lua, fi >= 0, 2,  "field index must be >= 0");
        let ai = luaL_optinteger(lua, 3, 0);
        argcheck(lua, ai >= 0, 3, "array index must be >= 0");
        let luserdata = lua_touserdata(lua, LUA_UPVALUEINDEX as i32);
        argcheck(lua, luserdata != std::ptr::null(), 0, "invalid lightuserdata");

        let lsb = luserdata as *mut LSB;
        let sandbox = lsb_get_parent(lsb) as *mut LuaSandbox;
        assert!(!sandbox.is_null());
        if (*sandbox).msg.is_none() { // read attempt outside process_message is non-fatal
            lua_pushnil(lua);
            return 1;
        }
        let msg = (*sandbox).msg.get_ref();


        let field = std::string::raw::from_buf(f as *const u8);

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
        argcheck(lua, lua_gettop(lua) == 0, 1, "takes no arguments");
        let luserdata = lua_touserdata(lua, LUA_UPVALUEINDEX as i32);
        argcheck(lua, luserdata != std::ptr::null(), 0, "invalid lightuserdata");

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
        argcheck(lua, lua_gettop(lua) == 1, 1, "takes a single string argument");
        let luserdata = lua_touserdata(lua, LUA_UPVALUEINDEX as i32);
        argcheck(lua, luserdata != std::ptr::null(), 0, "invalid lightuserdata");

        let lsb = luserdata as *mut LSB;
        let sandbox = lsb_get_parent(lsb) as *mut LuaSandbox;
        assert!(!sandbox.is_null());
        let sandbox: &mut LuaSandbox = &mut (*sandbox); // Get a Rust pointer

        // Get the config key as a Rust string
        let name: *const c_char = luaL_checklstring(lua, 1, std::ptr::mut_null());
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

fn update_field(lua: *mut LUA, varg: c_int, field: &mut pb::Field, ai: uint) {
    match field.get_value_type() {
        pb::Field_STRING => {
            let mut len: size_t = 0;
            let a = field.mut_value_string();
            let l = a.len();
            if ai <= l {
                unsafe {
                    let c: *const c_char = lua_tolstring(lua, varg, &mut len);
                    let v = std::string::raw::from_buf_len(c as *const u8, len as uint); // allow embedded nulls
                    if ai == l {
                        a.push(v);
                    } else {
                        *a.get_mut(ai) = v;
                    }
                }
            } else {
                argcheck(lua, false, 5, "invalid index");
            }
        }
        pb::Field_BYTES => {
            let mut len: size_t = 0;
            let a = field.mut_value_bytes();
            let l = a.len();
            if ai <= l {
                unsafe {
                    let c: *const c_char = lua_tolstring(lua, varg, &mut len);
                    let v = std::vec::raw::from_buf(c as *const u8, len as uint);
                    if ai == l {
                        a.push(v);
                    } else {
                        *a.get_mut(ai) = v;
                    }
                }
            } else {
                argcheck(lua, false, 5, "invalid index");
            }
        }
        pb::Field_INTEGER => {
            let a = field.mut_value_integer();
            let l = a.len();
            if ai <= l {
                unsafe {
                    let v = lua_tonumber(lua, varg) as i64;
                    if ai == l {
                        a.push(v);
                    } else {
                        *a.get_mut(ai) = v;
                    }
                }
            } else {
                argcheck(lua, false, 5, "invalid index");
            }
        }
        pb::Field_DOUBLE => {
            let a = field.mut_value_double();
            let l = a.len();
            if ai <= l {
                unsafe {
                    let v = lua_tonumber(lua, varg) as f64;
                    if ai == l {
                        a.push(v);
                    } else {
                        *a.get_mut(ai) = v;
                    }
                }
            } else {
                argcheck(lua, false, 5, "invalid index");
            }
        }
        pb::Field_BOOL => {
            let a = field.mut_value_bool();
            let l = a.len();
            if ai <= l {
                unsafe {
                    let v = lua_toboolean(lua, varg) == 1;
                    if ai == l {
                        a.push(v);
                    } else {
                        *a.get_mut(ai) = v;
                    }
                }
            } else {
                argcheck(lua, false, 5, "invald index");
            }
        }
    }
}

extern fn write_message(lua: *mut LUA) -> c_int {
    unsafe {
        let n = lua_gettop(lua);
        argcheck(lua, n > 1 && n < 6, 0, "incorrect number of arguments");
        let name: *const c_char = luaL_checklstring(lua, 1, std::ptr::mut_null());
        let t = lua_type(lua, 2);
        argcheck(lua, t == 4 || t == 3 || t == 1, 2, "only accepts string, numeric, or boolean values");
        let r: *const c_char = luaL_optlstring(lua, 3, "".as_ptr() as *const i8, std::ptr::mut_null());
        let fi = luaL_optinteger(lua, 4, 0);
        argcheck(lua, fi >= 0, 4,  "field index must be >= 0");
        let fi = fi as uint;
        let ai = luaL_optinteger(lua, 5, 0);
        argcheck(lua, ai >= 0, 5, "array index must be >= 0");
        let ai = ai as uint;
        let luserdata = lua_touserdata(lua, LUA_UPVALUEINDEX as i32);
        argcheck(lua, luserdata != std::ptr::null(), 0, "invalid lightuserdata");

        let lsb = luserdata as *mut LSB;
        let sandbox = lsb_get_parent(lsb) as *mut LuaSandbox;
        assert!(!sandbox.is_null());
        if (*sandbox).msg.is_none() {
            argcheck(lua, false, 0,  "no message available");
        }
        let msg = (*sandbox).msg.get_mut_ref();
        let field = std::string::raw::from_buf(name as *const u8);

        match field.as_slice() {
            "Type" => {
                argcheck(lua, t == 4, 2, "'Type' must be a string");
                let v: *const c_char = lua_tolstring(lua, 2, std::ptr::mut_null());
                msg.set_field_type(std::string::raw::from_buf(v as *const u8));
            }
            "Logger" => {
                argcheck(lua, t == 4, 2, "'Logger' must be a string");
                let v: *const c_char = lua_tolstring(lua, 2, std::ptr::mut_null());
                msg.set_logger(std::string::raw::from_buf(v as *const u8));
            }
            "Payload" => {
                argcheck(lua, t == 4, 2, "'Payload' must be a string");
                let v: *const c_char = lua_tolstring(lua, 2, std::ptr::mut_null());
                msg.set_payload(std::string::raw::from_buf(v as *const u8));
            }
            "EnvVersion" => {
                argcheck(lua, t == 4, 2, "'EnvVersion' must be a string");
                let v: *const c_char = lua_tolstring(lua, 2, std::ptr::mut_null());
                msg.set_env_version(std::string::raw::from_buf(v as *const u8));
            }
            "Hostname" => {
                argcheck(lua, t == 4, 2, "'Hostname' must be a string");
                let v: *const c_char = lua_tolstring(lua, 2, std::ptr::mut_null());
                msg.set_hostname(std::string::raw::from_buf(v as *const u8));
            }
//            "Uuid" => {
//                argcheck(lua, t == 4, 2, "'Uuid' must be a string");
//            }
            "Timestamp" => { // todo add date/time string conversion
                argcheck(lua, t == 3, 2, "'Timestamp' must be a number");
                msg.set_timestamp(lua_tonumber(lua, 2) as i64);
            }
            "Severity" => {
                argcheck(lua, t == 3 || t == 4, 2, "'Severity' must be a number or string");
                msg.set_severity(lua_tointeger(lua, 2) as i32);
            }
            "Pid" => {
                argcheck(lua, t == 3 || t == 4, 2, "'Pid' must be a number or string");
                msg.set_pid(lua_tointeger(lua, 2) as i32);
            }
            _ => {
                let t = match t {
                    4 => pb::Field_STRING,
                    3 => pb::Field_DOUBLE,
                    1 => pb::Field_BOOL,
                    _ => pb::Field_STRING
                };
                match get_field_name(field.as_slice())
                {
                    Some(name) => {
                        let mut nf:  Option<pb::Field> = None; // todo Brian better way to allow the add_fields() in None?
                        let r = std::string::raw::from_buf(r as *const u8);
                        match find_field_mut(msg, name, fi as uint)
                        {
                            (Some(f), _) => {
                                if t != f.get_value_type() {
                                    argcheck(lua, false, 1, "field type mis-match");
                                }
                                if f.has_representation() && r.len() == 0 {
                                    f.clear_representation();
                                } else if r.len() > 0 {
                                    f.set_representation(r);
                                }
                                update_field(lua, 2, f, ai);
                            }
                            (None, cnt) => {
                                if fi != cnt  {
                                    argcheck(lua, false, 4, "invalid field index");
                                }
                                if ai != 0 {
                                    argcheck(lua, false, 5, "invalid array index");
                                }
                                nf = Some(pb::Field::new());
                                nf.get_mut_ref().set_name(name.into_string());
                                nf.get_mut_ref().set_value_type(t);
                                if r.len() > 0 {
                                    nf.get_mut_ref().set_representation(r);
                                }
                                update_field(lua, 2, nf.get_mut_ref(), ai);
                            }
                        }
                        if nf.is_some() {
                            msg.add_fields(nf.unwrap());
                        }
                    }
                    None => {
                        argcheck(lua, false, 1,  "invalid field name");
                    }
                }
            }
        }
    }
    return 0;
}

#[cfg(test)]
mod test {
    use std::any::Any;
    use sandbox;
    use message::pb;

    #[test]
    fn creation_failed() {
        let mut sb = sandbox::LuaSandbox::new("".as_bytes(), "".as_bytes(), 32767, 1000, 9*1024*1024);
        // make sure all the functions properly handle the bad state
        assert!(sb.last_error().as_slice() == "creation failed");
        assert!(sb.state() as int == sandbox::STATE_UNKNOWN as int);
        assert!(-1 == sb.init("".as_bytes()));
        assert!(0 == sb.usage(sandbox::TYPE_MEMORY, sandbox::STAT_CURRENT));
        let mut m = Some(pb::HekaMessage::new());
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
        let mut m = Some(pb::HekaMessage::new());
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
        let mut m = Some(pb::HekaMessage::new());
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
        let mut f = pb::Field::new();
        f.set_name("test".into_string());
        f.set_value_type(pb::Field_STRING);
        f.add_value_string("foo".into_string());
        f.add_value_string("bar".into_string());
        m.get_mut_ref().add_fields(f);
        let mut f1 = pb::Field::new();
        f1.set_name("widget".into_string());
        f1.set_value_type(pb::Field_INTEGER);
        f1.add_value_integer(222);
        m.get_mut_ref().add_fields(f1);
        let mut f2 = pb::Field::new();
        f2.set_name("test".into_string());
        f2.set_value_type(pb::Field_STRING);
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
        let err = vec!["process_message() ../test/read_message_error.lua:8: bad argument #0 to 'read_message' (incorrect number of arguments)",
         "process_message() ../test/read_message_error.lua:10: bad argument #0 to 'read_message' (incorrect number of arguments)",
         "process_message() ../test/read_message_error.lua:12: bad argument #1 to 'read_message' (string expected, got table)",
         "process_message() ../test/read_message_error.lua:14: bad argument #2 to 'read_message' (field index must be >= 0)",
         "process_message() ../test/read_message_error.lua:16: bad argument #3 to 'read_message' (array index must be >= 0)"];
        let mut idx = 0;
        let mut m = Some(pb::HekaMessage::new());
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
        let mut m = Some(pb::HekaMessage::new());

        let mut f = pb::Field::new();
        f.set_name("foo".into_string());
        f.set_representation("test".into_string());
        f.set_value_type(pb::Field_STRING);
        f.add_value_string("bar".into_string());
        m.get_mut_ref().add_fields(f);


        let mut f1 = pb::Field::new();
        f1.set_name("bytes".into_string());
        f1.set_value_type(pb::Field_BYTES);
        f1.add_value_bytes(vec!['d' as u8, 'a' as u8, 't' as u8, 'a' as u8]);
        m.get_mut_ref().add_fields(f1);

        let mut f2 = pb::Field::new();
        f2.set_name("int".into_string());
        f2.set_value_type(pb::Field_INTEGER);
        f2.add_value_integer(999);
        f2.add_value_integer(1000);
        m.get_mut_ref().add_fields(f2);

        let mut f3 = pb::Field::new();
        f3.set_name("double".into_string());
        f3.set_value_type(pb::Field_DOUBLE);
        f3.add_value_double(99.9);
        m.get_mut_ref().add_fields(f3);

        let mut f4 = pb::Field::new();
        f4.set_name("bool".into_string());
        f4.set_value_type(pb::Field_BOOL);
        f4.add_value_bool(true);
        m.get_mut_ref().add_fields(f4);

        let mut f5 = pb::Field::new();
        f5.set_name("foo".into_string());
        f5.set_value_type(pb::Field_STRING);
        f5.add_value_string("alternate".into_string());
        m.get_mut_ref().add_fields(f5);

        let mut f6 = pb::Field::new();
        f6.set_name("false".into_string());
        f6.set_value_type(pb::Field_BOOL);
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
        assert!(sb.last_error().as_slice() == "timer_event() ../test/read_next_field.lua:31: bad argument #1 to 'read_next_field' (takes no arguments)", "received: {}", sb.last_error());
        assert!(sb.destroy("".as_bytes()).is_empty());
    }

    #[test]
    fn write_message() {
        let mut m = Some(pb::HekaMessage::new());
        let mut sb = sandbox::LuaSandbox::new("../test/write_message.lua".as_bytes(), "".as_bytes(), 64*1024, 0, 0);
        assert!(sb.last_error().is_empty());
        assert!(0 == sb.init("".as_bytes()));
        let (rc, mm) = sb.process_message(m.take_unwrap());
        m = Some(mm);
        assert!(rc == 0, "{}", sb.last_error());
        assert!(sb.destroy("".as_bytes()).is_empty());
        assert!(m.get_ref().get_field_type().as_slice() == "MyType");
        assert!(m.get_ref().get_logger().as_slice() == "MyLogger");
        assert!(m.get_ref().get_timestamp() == 999);
        // todo string Timestamp
        assert!(m.get_ref().get_payload().as_slice() == "MyPayload");
        assert!(m.get_ref().get_env_version().as_slice() == "000");
        assert!(m.get_ref().get_hostname().as_slice() == "MyHostname");
        assert!(m.get_ref().get_severity() == 4);
        assert!(m.get_ref().get_pid() == 12345);
        let f = m.get_ref().get_fields();
        assert!(f.len() == 6, "{}", f.len());
        assert!(f[0].get_value_type() == pb::Field_STRING, "{}", f[0].get_value_type() as int);
        assert!(f[0].get_name().as_slice() == "String");
        assert!(f[0].get_value_string().len() == 1);
        assert!(f[0].get_value_string()[0].as_slice() == "foo");
        assert!(!f[0].has_representation());
        assert!(f[1].get_value_type() == pb::Field_DOUBLE, "{}", f[1].get_value_type() as int);
        assert!(f[1].get_name().as_slice() == "Float");
        assert!(f[1].get_value_double().len() == 1);
        assert!(f[1].get_value_double()[0] == 1.2345);
        assert!(!f[1].has_representation());
        assert!(f[2].get_value_type() == pb::Field_DOUBLE, "{}", f[2].get_value_type() as int);
        assert!(f[2].get_name().as_slice() == "Int");
        assert!(f[2].get_value_double().len() == 2);
        assert!(f[2].get_value_double()[0] == 123f64);
        assert!(f[2].get_value_double()[1] == 456f64);
        assert!(f[2].get_representation().as_slice() == "count");
        assert!(f[3].get_value_type() == pb::Field_BOOL, "{}", f[3].get_value_type() as int);
        assert!(f[3].get_name().as_slice() == "Bool");
        assert!(f[3].get_value_bool().len() == 1);
        assert!(f[3].get_value_bool()[0] == true);
        assert!(!f[3].has_representation());
        assert!(f[4].get_value_type() == pb::Field_BOOL, "{}", f[4].get_value_type() as int);
        assert!(f[4].get_name().as_slice() == "Bool");
        assert!(f[4].get_value_bool().len() == 2);
        assert!(f[4].get_value_bool()[0] == false);
        assert!(f[4].get_value_bool()[1] == false);
        assert!(!f[4].has_representation());
        assert!(f[5].get_value_type() == pb::Field_STRING, "{}", f[5].get_value_type() as int);
        assert!(f[5].get_name().as_slice() == "");
        assert!(f[5].get_value_string().len() == 1);
        assert!(f[5].get_value_string()[0].as_slice() == "bad idea");
        assert!(!f[5].has_representation());
    }

    #[test]
    fn write_message_error() {
        let err = vec![
            "process_message() ../test/write_message_error.lua:8: bad argument #0 to 'write_message' (incorrect number of arguments)",
            "process_message() ../test/write_message_error.lua:10: bad argument #0 to 'write_message' (incorrect number of arguments)",
            "process_message() ../test/write_message_error.lua:12: bad argument #1 to 'write_message' (string expected, got nil)",
            "process_message() ../test/write_message_error.lua:14: bad argument #2 to 'write_message' (only accepts string, numeric, or boolean values)",
            "process_message() ../test/write_message_error.lua:16: bad argument #4 to 'write_message' (field index must be >= 0)",
            "process_message() ../test/write_message_error.lua:18: bad argument #5 to 'write_message' (array index must be >= 0)",
            "process_message() ../test/write_message_error.lua:20: bad argument #1 to 'write_message' (invalid field name)",
            "process_message() ../test/write_message_error.lua:22: bad argument #4 to 'write_message' (invalid field index)",
            "process_message() ../test/write_message_error.lua:24: bad argument #5 to 'write_message' (invalid array index)",
            "process_message() ../test/write_message_error.lua:26: bad argument #2 to 'write_message' ('Type' must be a string)",
            "process_message() ../test/write_message_error.lua:28: bad argument #2 to 'write_message' ('Logger' must be a string)",
            "process_message() ../test/write_message_error.lua:30: bad argument #2 to 'write_message' ('Payload' must be a string)",
            "process_message() ../test/write_message_error.lua:32: bad argument #2 to 'write_message' ('EnvVersion' must be a string)",
            "process_message() ../test/write_message_error.lua:34: bad argument #2 to 'write_message' ('Hostname' must be a string)",
            "process_message() ../test/write_message_error.lua:36: bad argument #2 to 'write_message' ('Timestamp' must be a number)",
            "process_message() ../test/write_message_error.lua:38: bad argument #2 to 'write_message' ('Severity' must be a number or string)",
            "process_message() ../test/write_message_error.lua:40: bad argument #2 to 'write_message' ('Pid' must be a number or string)",
            "process_message() ../test/write_message_error.lua:43: bad argument #1 to 'write_message' (field type mis-match)"
        ];
        let mut idx = 0;
        let mut m = Some(pb::HekaMessage::new());
        for e in err.iter() {
            let mut sb = sandbox::LuaSandbox::new("../test/write_message_error.lua".as_bytes(), "".as_bytes(), 64*1024, 0, 0);
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
}
