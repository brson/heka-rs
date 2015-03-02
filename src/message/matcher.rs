use regex::Regex;
use std;
use std::collections::{LinkedList};
use message;
use message::pb;
use uuid::Uuid;

use self::Value::{Text,Number,Boolean,Re, Nil};
use self::ExpectNode::{Conditional,LogicalOperator};
use self::Op::{Equal,NotEqual,GtEqual,Gt,LtEqual,Lt,ReEqual,ReNotEqual,True,False,Or,And,OpenParen};

enum Value {
    Text(String),
    Number(f64),
    Boolean(bool),
    Re(Regex),
    Nil,
}

#[derive(Copy)]
enum ExpectNode {
    Conditional,
    LogicalOperator,
}

#[derive(Copy)]
enum Op {
    Equal,
    NotEqual,
    GtEqual,
    Gt,
    LtEqual,
    Lt,
    ReEqual,
    ReNotEqual,

    True,
    False,

    Or,
    And,
    OpenParen,
}

struct Node {
    variable: String,
    op: Op,
    is_field: bool,
    fi: usize,
    ai: usize,
    value: Value,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

pub struct Matcher {
    spec: String,
    expect: ExpectNode,
    node: Box<Node>,
    msg: String,
    stack: LinkedList<Box<Node>>,
    output: LinkedList<Box<Node>>,
}

pub struct Error {
    pub pos: usize,
    pub msg: String,
}

impl Node {
    pub fn new() -> Node {
        Node {
            variable: "".to_string(),
            op: Equal,
            is_field: false,
            fi: 0,
            ai: 0,
            value: Nil,
            left: None,
            right: None,
        }
    }
}

impl Matcher {
    pub fn new(s: &str) -> Result<Matcher, Error> {
        let mut m = Matcher {
            spec: s.to_string(),
            expect: Conditional,
            node: Box::new(Node::new()),
            msg: "Failed Parsing".to_string(),
            stack: LinkedList::new(),
            output: LinkedList::new(),
        };

        let b = s.as_bytes();
        let l = b.len();
        let mut pos = 0us;

        loop {
            if pos >= l {
                break;
            }
            match b[pos] as char {
                '(' => {
                    pos += 1;
                    m.node.op = OpenParen;
                    m.stack.push_front(m.node);
                    m.expect = Conditional;
                }
                ')' => {
                    pos += 1;
                    if !m.pop_to_matching_paren() {
                        return Err(Error{pos: pos, msg: m.msg});
                    }
                    m.expect = LogicalOperator;
                }
                ' ' => {pos += 1;} // discard spaces
                '&' | '|' => {
                    if m.expect as isize != LogicalOperator as isize || !m.match_logical_op(s.slice_from(pos), &mut pos) {
                        return Err(Error{pos: pos, msg: m.msg});
                    }
                    if !m.pop_lower_precedence_ops() {
                        return Err(Error{pos: pos, msg: m.msg});
                    }
                    m.stack.push_front(m.node);
                    m.expect = Conditional;
                }
                _ => {
                    if m.expect as isize != Conditional as isize || !m.match_condition(s.slice_from(pos), &mut pos) {
                        return Err(Error{pos: pos, msg: m.msg});
                    }
                    m.output.push_back(m.node);
                    m.expect = LogicalOperator;
                }
            }
            m.node = Box::new(Node::new());
        }

        if m.pop_remaining_ops() && m.output.len() == 1 {
            match m.output.pop_back() {
                Some(n) => {
                    m.node = n;
                    Ok(m)
                }
                None => {
                    Err(Error{pos: pos, msg: m.msg})
                }
            }
        } else {
            Err(Error{pos: pos, msg: m.msg})
        }
    }

    pub fn is_match(&self, m: &pb::HekaMessage) -> bool {
        evaluate_tree(&self.node, m)
    }

    fn push_op(&mut self, n: Box<Node>) {
        let mut mn = n;
        mn.right = self.output.pop_back();
        mn.left = self.output.pop_back();
        self.output.push_back(mn);
    }

    fn pop_to_matching_paren(&mut self) -> bool {
         let mut matched = false;
         let mut count = 0us;
         loop {
             match self.stack.pop_front() {
                 Some(n) => {
                     match n.op {
                         OpenParen => {
                             matched = true;
                             break;
                         }
                         _ => {
                             self.push_op(n);
                             count += 1;
                         }
                     }
                 }
                 None => break
             }
         }
         if !matched {
             self.msg = "mis-matched parens".to_string();
             return false;
         }
         if count == 0 {
             self.msg = "empty parens".to_string();
             return false;
         }
         true
    }

    fn pop_lower_precedence_ops(&mut self) -> bool {
        loop {
            match self.stack.pop_front() {
                Some(n) => {
                    if n.op as isize != OpenParen as isize && self.node.op as isize <= n.op as isize {
                        self.push_op(n);
                    } else {
                        self.stack.push_front(n); // todo fix removing it, just to add it back on in this case
                        break;
                    }
                }
                None => break
            };
        }
        true
    }

    fn pop_remaining_ops(&mut self) -> bool {
        loop {
            match self.stack.pop_front() {
                Some(n) => {
                    if n.op as isize != OpenParen as isize {
                        self.push_op(n);
                    } else {
                        self.msg = "missing closing paren".to_string();
                        return false;
                    }
                }
                None => break
            }
        }
        true
    }

    fn match_op(&mut self, s: &str, pos: &mut usize) -> bool {
        let re = regex!(r"^\s*(==|!=|>=|>|<=|<)\s*");
        match re.captures(s) {
            Some(c) => {
                self.node.op = match c.at(1).unwrap() {
                    "==" => Equal,
                    "!=" => NotEqual,
                    ">=" => GtEqual,
                    ">" => Gt,
                    "<=" => LtEqual,
                    "<" => Lt,
                    _ => return false
                };
                let (_, e) = c.pos(0).unwrap();
                *pos += e;
                true
            },
            None => false
        }
    }

    fn match_re_op(&mut self, s: &str, pos: &mut usize) -> bool {
        let re = regex!(r"^\s*(=~|!~)\s*");
        match re.captures(s) {
            Some(c) => {
                self.node.op = match c.at(1).unwrap() {
                    "=~" => ReEqual,
                    "!~" => ReNotEqual,
                    _ => return false
                };
                let (_, e) = c.pos(0).unwrap();
                *pos += e;
                true
            },
            None => false
        }
    }

    fn match_logical_op(&mut self, s: &str, pos: &mut usize) -> bool {
        let re = regex!(r"^(&&|\|\|)");
        match re.captures(s) {
            Some(c) => {
                self.node.op = match c.at(1).unwrap() {
                    "&&" => And,
                    "||" => Or,
                    _ => return false
                };
                let (_, e) = c.pos(0).unwrap();
                *pos += e;
                true
            },
            None => false
        }
    }

    fn match_string_value(&mut self, s: &str, pos: &mut usize, allow_nil: bool) -> bool {
        let mut re = regex!(r#"('(?:\\'|[^'])*'|"(?:\\"|[^"])*")"#);
        if allow_nil {
            re = regex!(r#"(NIL|'(?:\\'|[^'])*'|"(?:\\"|[^"])*")"#);
        }

        match re.captures(s) {
            Some(c) => {
                self.node.value = match c.at(1).unwrap() {
                    "NIL" => Nil,
                    t => Text(t.slice(1, t.len()-1).to_string())
                };
                let (_, e) = c.pos(0).unwrap();
                *pos += e;
                true
            },
            None => false
        }
    }

    fn match_re_value(&mut self, s: &str, pos: &mut usize) -> bool {
        let re = regex!(r"^/((?:\\/|[^/])*)/");
        match re.captures(s) {
            Some(c) => {
                let (_, e) = c.pos(0).unwrap();
                *pos += e;
                self.node.value = match Regex::new(c.at(1).unwrap()) {
                    Ok(re) => Re(re),
                    Err(err) => {
                        self.msg = err.msg;
                        return false;
                    }
                };
                true
            },
            None => false
        }
    }

    fn match_integer_value(&mut self, s: &str, pos: &mut usize) -> bool {
        let re = regex!(r"^(\d+)");
        match re.captures(s) {
            Some(c) => {
                self.node.value = Number(std::str::FromStr::from_str(c.at(1).unwrap()).unwrap());
                let (_, e) = c.pos(0).unwrap();
                *pos += e;
                true
            },
            None => false
        }
    }

    fn match_numeric_value(&mut self, s: &str, pos: &mut usize, allow_nil: bool) -> bool {
        let mut re = regex!(r"^([+-]?\d+.\d+(?:[eE][+-]?d+)?|\d+)");
        if allow_nil {
            re = regex!(r"^(NIL|[+-]?\d+.\d+(?:[eE][+-]?d+)?|\d+)");
        }
        match re.captures(s) {
            Some(c) => {
                self.node.value = match c.at(1).unwrap() {
                    "NIL" => Nil,
                    n => Number(std::str::FromStr::from_str(n).unwrap())
                };
                let (_, e) = c.pos(0).unwrap();
                *pos += e;
                true
            },
            None => false
        }
    }

    fn match_boolean_value(&mut self, s: &str, pos: &mut usize) -> bool {
        let re = regex!("^(TRUE|FALSE)");
        match re.captures(s) {
            Some(c) => {
                self.node.value = match c.at(1).unwrap() {
                    "TRUE" => Boolean(true),
                    "FALSE" => Boolean(false),
                    _ => return false
                };
                let (_, e) = c.pos(0).unwrap();
                *pos += e;
                true
            },
            None => false
        }
    }

    fn match_string_expression(&mut self, s: &str, pos: &mut usize) -> bool {
        let re = regex!("^(Type|Logger|Hostname|EnvVersion|Payload|Uuid)");
        match re.captures(s) {
            Some(c) => {
                self.node.variable = c.at(1).unwrap().to_string();
                let (_, e) = c.pos(0).unwrap();
                *pos += e;
                let lpos = *pos;
                let s = s.slice_from(e);
                if self.match_op(s, pos) {
                    if !self.match_string_value(s.slice_from(*pos-lpos), pos, false) {
                        return false;
                    }
                } else if self.match_re_op(s, pos) {
                    if !self.match_re_value(s.slice_from(*pos-lpos), pos) {
                        return false;
                    }
                }
                true
            },
            None => false
        }
    }

    fn match_integer_expression(&mut self, s: &str, pos: &mut usize) -> bool {
        let re = regex!("^(Timestamp|Severity|Pid)");
        match re.captures(s) {
            Some(c) => {
                self.node.variable = c.at(1).unwrap().to_string();
                let (_, e) = c.pos(0).unwrap();
                *pos += e;
                let lpos = *pos;
                let s = s.slice_from(e);
                if self.match_op(s, pos) {
                    if !self.match_integer_value(s.slice_from(*pos-lpos), pos) {
                        return false;
                    }
                }
                true
            },
            None => false
        }
    }

    fn match_field_expression(&mut self, s: &str, pos: &mut usize) -> bool {
        let re = regex!(r"^Fields\[([^]]*?)\](?:\[(\d+)\])?(?:\[(\d+)\])?");
        match re.captures(s) {
            Some(c) => {
                self.node.variable = c.at(1).unwrap().to_string();
                self.node.is_field = true;
                self.node.fi = match c.at(2) {
                    Some(u) => match std::str::FromStr::from_str(u) {
                        Ok(i) => i,
                        Err(e) => 0
                    },
                    None => 0
                };
                self.node.ai = match c.at(3) {
                    Some(u) => match std::str::FromStr::from_str(u) {
                        Ok(i) => i,
                        Err(e) => 0
                    },
                    None => 0
                };
                let (_, e) = c.pos(0).unwrap();
                *pos += e;
                let lpos = *pos;
                let s = s.slice_from(e);
                if self.match_op(s, pos) {
                    let s = s.slice_from(*pos-lpos);
                    let equality = match self.node.op {
                        Equal | NotEqual => true,
                        _ => false
                    };
                    if !self.match_string_value(s, pos, equality) {
                        if !self.match_numeric_value(s, pos, equality) {
                            if !equality || !self.match_boolean_value(s, pos) {
                                return false;
                            }
                        }
                    }
                } else if self.match_re_op(s, pos) {
                    if !self.match_re_value(s.slice_from(*pos-lpos), pos) {
                        return false;
                    }
                }
                true
            },
            None => false
        }
    }

    fn match_boolean_expression(&mut self, s: &str, pos: &mut usize) -> bool {
        let re = regex!("^(TRUE|FALSE)");
        match re.captures(s) {
            Some(c) => {
                self.node.op = match c.at(1).unwrap() {
                    "TRUE" => True,
                    "FALSE" => False,
                    _ => return false
                };
                let (_, e) = c.pos(0).unwrap();
                *pos += e;
                true
            },
            None => false
        }
    }

    fn match_condition(&mut self, s: &str, pos: &mut usize) -> bool {
        if self.match_string_expression(s, pos) {
            return true;
        } else if self.match_integer_expression(s, pos) {
            return true;
        } else if self.match_field_expression(s, pos) {
            return true;
        } else if self.match_boolean_expression(s, pos) {
            return true;
        }
        false
    }
}

fn evaluate_tree(n: &Box<Node>, m: &pb::HekaMessage) -> bool {
    let r = match n.left {
        Some(ref ln) => evaluate_tree(ln, m),
        None => return evaluate_node(n, m)
    };
    if r == true && n.op as isize == Or as isize {
        return r; // short circuit
    }
    if r == false && n.op as isize == And as isize {
        return r; // short circuit
    }

    match n.right {
        Some(ref rn) => evaluate_tree(rn, m),
        None => r
    }
}

fn evaluate_node(n: &Box<Node>, m: &pb::HekaMessage) -> bool {
    match n.op {
        True => true,
        False =>false,
        _ => {
            if !n.is_field {
                match n.variable.as_slice() {
                    "Type" => test_string(m.get_field_type(), n),
                    "Logger" => test_string(m.get_logger(), n),
                    "Hostname" => test_string(m.get_hostname(), n),
                    "EnvVersion" => test_string(m.get_env_version(), n),
                    "Severity" => test_number(m.get_severity() as f64, n),
                    "Timestamp" => test_number(m.get_timestamp() as f64, n),
                    "Payload" => test_string(m.get_payload(), n),
                    "Pid" => test_number(m.get_pid() as f64, n),
                    "Uuid" => {
                        match Uuid::from_bytes(m.get_uuid()) {
                            Some(u) => {
                                test_string(u.to_hyphenated_string().as_slice(), n)
                            },
                            None => test_string("", n),
                        }
                    },
                    _ => false,
                }
            } else {
                match n.value { // todo figure out what we are doing with byte array comparison, for now it will always be false
                    Text(ref s) => {
                        let a = message::get_field_string(m, n.variable.as_slice(), n.fi, n.ai);
                        match a {
                            Some(a) => compare_string(&n.op, a.as_slice(), s.as_slice()),
                            None => false,
                        }
                    },
                    Number(f) => {
                        let a = message::get_field_number(m, n.variable.as_slice(), n.fi, n.ai);
                        match a {
                            Some(a) => compare_number(&n.op, a, f),
                            None => false,
                        }
                    },
                    Re(ref r) => {
                        let a = message::get_field_string(m, n.variable.as_slice(), n.fi, n.ai);
                        match a {
                            Some(a) => compare_re(&n.op, a.as_slice(), r),
                            None => false,
                        }
                    },
                    Boolean(b) => {
                        let a = message::get_field_bool(m, n.variable.as_slice(), n.fi, n.ai);
                        match a {
                            Some(a) => {
                                match n.op {
                                    Equal => a == b,
                                    NotEqual => a != b,
                                    _ => panic!("invalid bool comparison operator"),
                                }
                            },
                            None => false,
                        }
                    },
                    Nil => {
                        let r = match message::find_field(m, n.variable.as_slice(), n.fi)
                        {
                            Some(f) => {
                                match f.get_value_type() {
                                    pb::Field_ValueType::STRING =>  n.ai >= f.get_value_string().len(),
                                    pb::Field_ValueType::BYTES => n.ai >= f.get_value_bytes().len(),
                                    pb::Field_ValueType::INTEGER => n.ai >= f.get_value_integer().len(),
                                    pb::Field_ValueType::DOUBLE => n.ai >= f.get_value_double().len(),
                                    pb::Field_ValueType::BOOL => n.ai >= f.get_value_bool().len(),
                                }
                            },
                            None => true,
                        };
                        match n.op {
                            Equal => r,
                            NotEqual => !r,
                            _ => panic!("invalid NIL comparison operator"),
                        }
                    },
                }
            }
        }
    }
}

#[inline(always)]
fn compare_string(op: &Op, a: &str, b: &str) -> bool {
    match op {
        &Equal => a == b,
        &NotEqual => a != b,
        &Lt => a < b,
        &LtEqual => a <= b,
        &Gt => a > b,
        &GtEqual => a >= b,
        _ => panic!("invalid string comparison operator"),
    }
}

#[inline(always)]
fn compare_re(op: &Op, a: &str, b: &Regex) -> bool {
    match op {
        &ReEqual => b.is_match(a),
        &ReNotEqual => !b.is_match(a),
        _ => panic!("invalid re comparison operator"),
    }
}

#[inline(always)]
fn compare_number(op: &Op, a: f64, b: f64) -> bool {
    match op {
        &Equal => a == b,
        &NotEqual => a != b,
        &Lt => a < b,
        &LtEqual => a <= b,
        &Gt => a > b,
        &GtEqual => a >= b,
        _ => panic!("invalid number comparison operator"),
    }
}

fn test_string(a: &str, n: &Box<Node>) -> bool {
    match n.value {
        Text(ref s) => {
            compare_string(&n.op, a, s.as_slice())
        },
        Re(ref r) => {
            compare_re(&n.op, a, r)
        },
        _ => panic!("unexpected value for test_string"),
    }
}

fn test_number(a: f64, n: &Box<Node>) -> bool {
    match n.value {
        Number(f) => {
            compare_number(&n.op, a, f)
        },
        _ => panic!("unexpected value for test_number"),
    }
}

#[cfg(test)]
mod test {
    extern crate test;
    use message::matcher;
    use message::pb;
    use uuid::Uuid;

    fn add_field_integer(m: &mut pb::HekaMessage, name: &str, val: i64) {
        let mut f = pb::Field::new();
        f.set_value_type(pb::Field_ValueType::INTEGER);
        f.set_name(name.to_string());
        f.mut_value_integer().push(val);
        if val == 999 {
            f.mut_value_integer().push(1024);
        }
        m.mut_fields().push(f);
    }

    fn add_field_double(m: &mut pb::HekaMessage, name: &str, val: f64) {
        let mut f = pb::Field::new();
        f.set_value_type(pb::Field_ValueType::DOUBLE);
        f.set_name(name.to_string());
        f.mut_value_double().push(val);
        m.mut_fields().push(f);
    }

    fn add_field_bool(m: &mut pb::HekaMessage, name: &str, val: bool) {
        let mut f = pb::Field::new();
        f.set_value_type(pb::Field_ValueType::BOOL);
        f.set_name(name.to_string());
        f.mut_value_bool().push(val);
        m.mut_fields().push(f);
    }

    fn add_field_string(m: &mut pb::HekaMessage, name: &str, val: &str) {
        let mut f = pb::Field::new();
        f.set_value_type(pb::Field_ValueType::STRING);
        f.set_name(name.to_string());
        f.mut_value_string().push(val.to_string());
        m.mut_fields().push(f);
    }

    fn add_field_bytes(m: &mut pb::HekaMessage, name: &str, val: Vec<u8>) {
        let mut f = pb::Field::new();
        f.set_value_type(pb::Field_ValueType::STRING);
        f.set_name(name.to_string());
        f.mut_value_bytes().push(val);
        m.mut_fields().push(f);
    }

    fn get_test_message() -> pb::HekaMessage {
        let mut msg = pb::HekaMessage::new();
        let u = match Uuid::parse_str("f47ac10b-58cc-4372-a567-0e02b2c3d479") {
            Ok(u) => u,
            Err(_) => panic!("bad uuid"),
        };
        msg.set_uuid(u.as_bytes().to_vec());
        msg.set_timestamp(9000000000);
        msg.set_field_type("TEST".to_string());
        msg.set_severity(6);
        msg.set_payload("Test Payload".to_string());
        msg.set_env_version("0.8".to_string());
        msg.set_pid(1234);
        msg.set_logger("UnitTest".to_string());
        msg.set_hostname("example.com".to_string());
        add_field_string(&mut msg, "foo", "bar");
        add_field_integer(&mut msg, "number", 64);
        add_field_bytes(&mut msg, "data", b"data".to_vec());
        add_field_integer(&mut msg, "int", 999);
        add_field_double(&mut msg, "double", 99.9);
        add_field_bool(&mut msg, "bool", true);
        add_field_string(&mut msg, "foo", "alternate");
        add_field_string(&mut msg, "Payload", "name=test;type=web;");
        add_field_integer(&mut msg, "Timestamp", 99000000000);
        add_field_integer(&mut msg, "zero", 0);
        add_field_string(&mut msg, "string", "43");
        return msg
    }

    fn test_match(m: &pb::HekaMessage, s: &str) -> bool {
        let mm = match matcher::Matcher::new(s) {
            Ok(m) => m,
            Err(e) => panic!("{}", e.msg),
        };
        mm.is_match(m)
    }

    #[test]
    fn failed_creation() {
        assert!(matcher::Matcher::new("").is_err());
        assert!(matcher::Matcher::new("  ").is_err());
        assert!(matcher::Matcher::new("bogus").is_err());
        assert!(matcher::Matcher::new("Type = 'test'").is_err());                                               // invalid operator
        assert!(matcher::Matcher::new("Pid == 'test='").is_err());                                              // Pid is not a string
        assert!(matcher::Matcher::new("Type == 'test' && (Severity==7 || Payload == 'Test Payload'").is_err()); // missing paren
        assert!(matcher::Matcher::new("Invalid == 'bogus'").is_err());                                          // unknown variable name
        assert!(matcher::Matcher::new("Fields[test][]").is_err());                                              // empty field index
        assert!(matcher::Matcher::new("Fields[test][a]").is_err());                                             // non numeric field index
        assert!(matcher::Matcher::new("Fields[test][0][]").is_err());                                           // empty array index
        assert!(matcher::Matcher::new("Fields[test][0][a]").is_err());                                          // non numeric array index
        assert!(matcher::Matcher::new("Fields[test][0][0][]").is_err());                                        // extra index dimension
        assert!(matcher::Matcher::new("Fields[test][xxxx").is_err());                                           // unmatched bracket
        assert!(matcher::Matcher::new("Pid =~ /6/").is_err());                                                  // regex not allowed on numeric
        assert!(matcher::Matcher::new("Pid !~ /6/").is_err());                                                  // regex not allowed on numeric
        assert!(matcher::Matcher::new("Type =~ /test").is_err());                                               // unmatched slash
        assert!(matcher::Matcher::new("Type == /test/").is_err());                                              // incorrect operator
        assert!(matcher::Matcher::new("Type =~ 'test'").is_err());                                              // string instead of regexp
        assert!(matcher::Matcher::new("Type =~ /\\ytest/").is_err());                                           // invalid escape character
        assert!(matcher::Matcher::new("Type != 'test\"").is_err());                                             // mis matched quote types
        assert!(matcher::Matcher::new("Pid =~ 6").is_err());                                                    // number instead of regexp
        assert!(matcher::Matcher::new("NIL").is_err());                                                         // invalid use of constant
        assert!(matcher::Matcher::new("Type == NIL").is_err());                                                 // existence check only works on fields
        assert!(matcher::Matcher::new("Fields[test] > TRUE").is_err());                                         // bool check only works with equals and not equals
        assert!(matcher::Matcher::new("Fields[test] > NIL").is_err());                                          // existence check only works with equals and not equals
        assert!(matcher::Matcher::new("&& TRUE").is_err());
        assert!(matcher::Matcher::new("Pid == 6 Severity == 7").is_err());
        assert!(matcher::Matcher::new("(  )").is_err());
    }

    #[test]
    fn failed_match() {
        let msg = get_test_message();
        assert!(!test_match(&msg, "FALSE"));
        assert!(!test_match(&msg, "Type == 'test'&&(Severity==7||Payload=='Test Payload')"));
        assert!(!test_match(&msg, "EnvVersion == '0.9'"));
        assert!(!test_match(&msg, "EnvVersion != '0.8'"));
        assert!(!test_match(&msg, "EnvVersion > '0.9'"));
        assert!(!test_match(&msg, "EnvVersion >= '0.9'"));
        assert!(!test_match(&msg, "EnvVersion < '0.8'"));
        assert!(!test_match(&msg, "EnvVersion <= '0.7'"));
        assert!(!test_match(&msg, "Severity == 5"));
        assert!(!test_match(&msg, "Severity != 6"));
        assert!(!test_match(&msg, "Severity < 6"));
        assert!(!test_match(&msg, "Severity <= 5"));
        assert!(!test_match(&msg, "Severity > 6"));
        assert!(!test_match(&msg, "Severity >= 7"));
        assert!(!test_match(&msg, "Fields[foo] == 'ba'"));
        assert!(!test_match(&msg, "Fields[foo][1] == 'bar'"));
        assert!(!test_match(&msg, "Fields[foo][0][1] == 'bar'"));
        assert!(!test_match(&msg, "Fields[bool] == FALSE"));
        assert!(!test_match(&msg, "Type =~ /Test/"));
        assert!(!test_match(&msg, "Type !~ /TEST/"));
        assert!(!test_match(&msg, "Payload =~ /^Payload/"));
        assert!(!test_match(&msg, "Type == \"te'st\""));
        assert!(!test_match(&msg, "Type == 'te\"st'"));
        assert!(!test_match(&msg, "Fields[int] =~ /999/"));
        assert!(!test_match(&msg, "Fields[zero] == \"0\""));
        assert!(!test_match(&msg, "Fields[string] == 43"));
        assert!(!test_match(&msg, "Fields[int] == NIL"));
        assert!(!test_match(&msg, "Fields[int][0][1] == NIL"));
        assert!(!test_match(&msg, "Fields[missing] != NIL"));
    }

    #[test]
    fn successful_match() {
        let msg = get_test_message();
        assert!(test_match(&msg, "TRUE"));
        assert!(test_match(&msg, "(Severity == 7 || Payload == 'Test Payload') && Type == 'TEST'"));
        assert!(test_match(&msg, "EnvVersion==\"0.8\""));
        assert!(test_match(&msg, " EnvVersion  ==  '0.8' "));
        assert!(test_match(&msg, "EnvVersion != '0.9'"));
        assert!(test_match(&msg, "EnvVersion > '0.7'"));
        assert!(test_match(&msg, "EnvVersion >= '0.8'"));
        assert!(test_match(&msg, "EnvVersion < '0.9'"));
        assert!(test_match(&msg, "EnvVersion <= '0.8'"));
        assert!(test_match(&msg, "Hostname != ''"));
        assert!(test_match(&msg, "Logger == 'UnitTest'"));
        assert!(test_match(&msg, "Pid != 0"));
        assert!(test_match(&msg, "Severity != 5"));
        assert!(test_match(&msg, "Severity < 7"));
        assert!(test_match(&msg, "Severity <= 6"));
        assert!(test_match(&msg, "Severity == 6"));
        assert!(test_match(&msg, "Severity > 5"));
        assert!(test_match(&msg, "Severity >= 6"));
        assert!(test_match(&msg, "Timestamp > 0"));
        assert!(test_match(&msg, "Type != 'test'"));
        assert!(test_match(&msg, "Type == 'TEST' && Severity == 6"));
        assert!(test_match(&msg, "Type == 'test' && Severity == 7 || Payload == 'Test Payload'"));
        assert!(test_match(&msg, "Type == 'TEST'"));
        assert!(test_match(&msg, "Type == 'foo' || Type == 'bar' || Type == 'TEST'"));
        assert!(test_match(&msg, "Uuid == 'f47ac10b-58cc-4372-a567-0e02b2c3d479'"));
        assert!(test_match(&msg, "Fields[foo] == 'bar'"));
        assert!(test_match(&msg, "Fields[foo][0] == 'bar'"));
        assert!(test_match(&msg, "Fields[foo][0][0] == 'bar'"));
        assert!(test_match(&msg, "Fields[foo][1] == 'alternate'"));
        assert!(test_match(&msg, "Fields[foo][1][0] == 'alternate'"));
        assert!(test_match(&msg, "Fields[foo] == 'bar'"));
//        assert!(test_match(&msg, "Fields[bytes] == 'data'"));
        assert!(test_match(&msg, "Fields[int] == 999"));
        assert!(test_match(&msg, "Fields[int][0][1] == 1024"));
        assert!(test_match(&msg, "Fields[double] == 99.9"));
        assert!(test_match(&msg, "Fields[bool] == TRUE"));
        assert!(test_match(&msg, "Type =~ /TEST/"));
        assert!(test_match(&msg, "Type !~ /bogus/"));
        assert!(test_match(&msg, "Type =~ /TEST/ && Payload =~ /Payload/"));
        assert!(test_match(&msg, "Fields[foo][1] =~ /alt/"));
        assert!(test_match(&msg, "Fields[Payload] =~ /name=\\w+/"));
        assert!(test_match(&msg, "Type =~ /(ST)/"));
        assert!(test_match(&msg, "Fields[int] != NIL"));
        assert!(test_match(&msg, "Fields[int][0][1] != NIL"));
        assert!(test_match(&msg, "Fields[int][0][2] == NIL"));
        assert!(test_match(&msg, "Fields[missing] == NIL"));
    }

    #[bench]
    fn create(b: &mut test::Bencher) {
        b.iter(|| matcher::Matcher::new("Type == 'Test' && Severity == 6"));
    }

    #[bench]
    fn match_simple(b: &mut test::Bencher) {
        let msg = get_test_message();
        let mm = match matcher::Matcher::new("Type == 'TEST' && Severity == 6") {
            Ok(m) => m,
            Err(e) => panic!("{}", e.msg),
        };
        b.iter(|| mm.is_match(&msg));
    }

    #[bench]
    fn match_re(b: &mut test::Bencher) {
        let msg = get_test_message();
        let mm = match matcher::Matcher::new("Type =~ /^TEST/ && Severity == 6") {
            Ok(m) => m,
            Err(e) => panic!("{}", e.msg),
        };
        b.iter(|| mm.is_match(&msg));
    }

    #[bench]
    fn match_re_capture(b: &mut test::Bencher) {
        let msg = get_test_message();
        let mm = match matcher::Matcher::new("Type =~ /^(TEST)/ && Severity == 6") {
            Ok(m) => m,
            Err(e) => panic!("{}", e.msg),
        };
        b.iter(|| mm.is_match(&msg));
    }

    #[bench]
    fn match_field_string(b: &mut test::Bencher) {
        let msg = get_test_message();
        let mm = match matcher::Matcher::new("Fields[foo] == 'bar' && Severity == 6") {
            Ok(m) => m,
            Err(e) => panic!("{}", e.msg),
        };
        b.iter(|| mm.is_match(&msg));
    }

    #[bench]
    fn match_field_number(b: &mut test::Bencher) {
        let msg = get_test_message();
        let mm = match matcher::Matcher::new("Fields[number] == 64 && Severity == 6") {
            Ok(m) => m,
            Err(e) => panic!("{}", e.msg),
        };
        b.iter(|| mm.is_match(&msg));
    }

    #[bench]
    fn match_field_non_existence(b: &mut test::Bencher) {
        let msg = get_test_message();
        let mm = match matcher::Matcher::new("Fields[missing] == NIL") {
            Ok(m) => m,
            Err(e) => panic!("{}", e.msg),
        };
        b.iter(|| mm.is_match(&msg));
    }

    #[bench]
    fn match_field_existence(b: &mut test::Bencher) {
        let msg = get_test_message();
        let mm = match matcher::Matcher::new("Fields[int] != NIL") {
            Ok(m) => m,
            Err(e) => panic!("{}", e.msg),
        };
        b.iter(|| mm.is_match(&msg));
    }
}

