use regex::Regex;
use std;
use std::collections::{DList, Deque};
use message;
use message::pb;

enum Value {
    Text(String),
    Number(f64),
    Boolean(bool),
    Re(Regex),
    Nil,
}

enum ExpectNode {
    Conditional,
    LogicalOperator,
}

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
    fi: uint,
    ai: uint,
    value: Value,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

pub struct Matcher {
    spec: String,
    expect: ExpectNode,
    node: Box<Node>,
    msg: String,
    stack: DList<Box<Node>>,
    output: DList<Box<Node>>,
}

pub struct Error {
    pub pos: uint,
    pub msg: String,
}

impl Node {
    pub fn new() -> Node {
        Node {
            variable: "".into_string(),
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
            spec: s.into_string(),
            expect: Conditional,
            node: box Node::new(),
            msg: "Failed Parsing".into_string(),
            stack: DList::new(),
            output: DList::new(),
        };

        let b = s.as_bytes();
        let l = b.len();
        let mut pos = 0u;

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
                    if m.expect as int != LogicalOperator as int || !m.match_logical_op(s.slice_from(pos), &mut pos) {
                        return Err(Error{pos: pos, msg: m.msg});
                    }
                    if !m.pop_lower_precedence_ops() {
                        return Err(Error{pos: pos, msg: m.msg});
                    }
                    m.stack.push_front(m.node);
                    m.expect = Conditional;
                }
                _ => {
                    if m.expect as int != Conditional as int || !m.match_condition(s.slice_from(pos), &mut pos) {
                        return Err(Error{pos: pos, msg: m.msg});
                    }
                    m.output.push(m.node);
                    m.expect = LogicalOperator;
                }
            }
            m.node = box Node::new();
        }

        if m.pop_remaining_ops() && m.output.len() == 1 {
            match m.output.pop() {
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
        mn.right = self.output.pop();
        mn.left = self.output.pop();
        self.output.push(mn);
    }

    fn pop_to_matching_paren(&mut self) -> bool {
         let mut matched = false;
         let mut count = 0u;
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
             self.msg = "mis-matched parens".into_string();
             return false;
         }
         if count == 0 {
             self.msg = "empty parens".into_string();
             return false;
         }
         true
    }

    fn pop_lower_precedence_ops(&mut self) -> bool {
        loop {
            match self.stack.pop_front() {
                Some(n) => {
                    if n.op as int != OpenParen as int && self.node.op as int <= n.op as int {
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
                    if n.op as int != OpenParen as int {
                        self.push_op(n);
                    } else {
                        self.msg = "missing closing paren".into_string();
                        return false;
                    }
                }
                None => break
            }
        }
        true
    }

    fn match_op(&mut self, s: &str, pos: &mut uint) -> bool {
        let re = regex!(r"^\s*(==|!=|>=|>|<=|<)\s*");
        match re.captures(s) {
            Some(c) => {
                self.node.op = match c.at(1) {
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

    fn match_re_op(&mut self, s: &str, pos: &mut uint) -> bool {
        let re = regex!(r"^\s*(=~|!~)\s*");
        match re.captures(s) {
            Some(c) => {
                self.node.op = match c.at(1) {
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

    fn match_logical_op(&mut self, s: &str, pos: &mut uint) -> bool {
        let re = regex!(r"^(&&|\|\|)");
        match re.captures(s) {
            Some(c) => {
                self.node.op = match c.at(1) {
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

    fn match_string_value(&mut self, s: &str, pos: &mut uint, allow_nil: bool) -> bool {
        let mut re = regex!(r#"('(?:\\'|[^'])*'|"(?:\\"|[^"])*")"#);
        if allow_nil {
            re = regex!(r#"(NIL|'(?:\\'|[^'])*'|"(?:\\"|[^"])*")"#);
        }

        match re.captures(s) {
            Some(c) => {
                self.node.value = match c.at(1) {
                    "NIL" => Nil,
                    t => Text(t.slice(1, t.len()-1).into_string())
                };
                let (_, e) = c.pos(0).unwrap();
                *pos += e;
                true
            },
            None => false
        }
    }

    fn match_re_value(&mut self, s: &str, pos: &mut uint) -> bool {
        let re = regex!(r"^/((?:\\/|[^/])*)/");
        match re.captures(s) {
            Some(c) => {
                let (_, e) = c.pos(0).unwrap();
                *pos += e;
                self.node.value = match Regex::new(c.at(1)) {
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

    fn match_integer_value(&mut self, s: &str, pos: &mut uint) -> bool {
        let re = regex!(r"^(\d+)");
        match re.captures(s) {
            Some(c) => {
                self.node.value = Number(std::from_str::from_str::<f64>(c.at(1)).unwrap());
                let (_, e) = c.pos(0).unwrap();
                *pos += e;
                true
            },
            None => false
        }
    }

    fn match_numeric_value(&mut self, s: &str, pos: &mut uint, allow_nil: bool) -> bool {
        let mut re = regex!(r"^([+-]?\d+.\d+(?:[eE][+-]?d+)?|\d+)");
        if allow_nil {
            re = regex!(r"^(NIL|[+-]?\d+.\d+(?:[eE][+-]?d+)?|\d+)");
        }
        match re.captures(s) {
            Some(c) => {
                self.node.value = match c.at(1) {
                    "NIL" => Nil,
                    n => Number(std::from_str::from_str::<f64>(n).unwrap())
                };
                let (_, e) = c.pos(0).unwrap();
                *pos += e;
                true
            },
            None => false
        }
    }

    fn match_boolean_value(&mut self, s: &str, pos: &mut uint) -> bool {
        let re = regex!("^(TRUE|FALSE)");
        match re.captures(s) {
            Some(c) => {
                self.node.value = match c.at(1) {
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

    fn match_string_expression(&mut self, s: &str, pos: &mut uint) -> bool {
        let re = regex!("^(Type|Logger|Hostname|EnvVersion|Payload|Uuid)");
        match re.captures(s) {
            Some(c) => {
                self.node.variable = c.at(1).into_string();
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

    fn match_integer_expression(&mut self, s: &str, pos: &mut uint) -> bool {
        let re = regex!("^(Timestamp|Severity|Pid)");
        match re.captures(s) {
            Some(c) => {
                self.node.variable = c.at(1).into_string();
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

    fn match_field_expression(&mut self, s: &str, pos: &mut uint) -> bool {
        let re = regex!(r"^Fields\[([^]]*?)\](?:\[(\d+)\])?(?:\[(\d+)\])?");
        match re.captures(s) {
            Some(c) => {
                self.node.variable = c.at(1).into_string();
                self.node.is_field = true;
                self.node.fi = match std::from_str::from_str::<uint>(c.at(2)) {
                    Some(u) => u,
                    None => 0
                };
                self.node.ai = match std::from_str::from_str::<uint>(c.at(3)) {
                    Some(u) => u,
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

    fn match_boolean_expression(&mut self, s: &str, pos: &mut uint) -> bool {
        let re = regex!("^(TRUE|FALSE)");
        match re.captures(s) {
            Some(c) => {
                self.node.op = match c.at(1) {
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

    fn match_condition(&mut self, s: &str, pos: &mut uint) -> bool {
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
    if r == true && n.op as int == Or as int {
        return r; // short circuit
    }
    if r == false && n.op as int == And as int {
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
                    "Uuid" => false,// return test_string(m.get_uuid(), n); // todo convert this to a string for comparison
                    _ => false,
                }
            } else {
                // todo may just want to match on the actual field type and call the test_x functions
                match n.value { // todo figure out what we are doing with byte array comparison
                    Text(ref s) => {
                        let a = message::get_field_string(m, n.variable.as_slice(), n.fi, n.ai);
                        match a {
                            Some(a) => { // todo remove duplicate test code
                                match n.op {
                                    Equal => a == s,
                                    NotEqual => a != s,
                                    Lt => a < s,
                                    LtEqual => a <= s,
                                    Gt => a > s,
                                    GtEqual => a >= s,
                                    _ => false,
                                }
                            },
                            None => false,
                        }
                    },
                    Number(f) => {
                        let a = message::get_field_number(m, n.variable.as_slice(), n.fi, n.ai);
                        match a {
                            Some(a) => { // todo remove duplicate test code
                                match n.op {
                                    Equal => f == a,
                                    NotEqual => f != a,
                                    Lt => f < a,
                                    LtEqual => f <= a,
                                    Gt => f > a,
                                    GtEqual => f >= a,
                                    _ => false,
                                }
                            },
                            None => false,
                        }
                    },
                    Re(ref r) => {
                        let a = message::get_field_string(m, n.variable.as_slice(), n.fi, n.ai);
                        match a {
                            Some(a) => { // todo remove duplicate test code
                                match n.op {
                                    ReEqual => r.is_match(a.as_slice()),
                                    ReNotEqual => !r.is_match(a.as_slice()),
                                    _ => false,
                                }
                            },
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
                                    _ => false,
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
                                    pb::Field_STRING =>  n.ai >= f.get_value_string().len(),
                                    pb::Field_BYTES => n.ai >= f.get_value_bytes().len(),
                                    pb::Field_INTEGER => n.ai >= f.get_value_integer().len(),
                                    pb::Field_DOUBLE => n.ai >= f.get_value_double().len(),
                                    pb::Field_BOOL => n.ai >= f.get_value_bool().len(),
                                }
                            },
                            None => true,
                        };
                        match n.op {
                            Equal => r,
                            NotEqual => !r,
                            _ => fail!("invalid NIL comparison"),
                        }
                    },
                }
            }
        }
    }
}

fn test_string(a: &str, n: &Box<Node>) -> bool {
    match n.value {
        Text(ref s) => {
            match n.op {
                Equal => a == s.as_slice(),
                NotEqual => a != s.as_slice(),
                Lt => a < s.as_slice(),
                LtEqual => a <= s.as_slice(),
                Gt => a > s.as_slice(),
                GtEqual => a >= s.as_slice(),
                _ => false,
            }
        },
        Re(ref r) => {
            match n.op {
                ReEqual => r.is_match(a),
                ReNotEqual => !r.is_match(a),
                _ => false,
            }
        },
        _ => false,
    }
}

fn test_number(f: f64, n: &Box<Node>) -> bool {
    match n.value {
        Number(v) => {
            match n.op {
                Equal => f == v,
                NotEqual => f != v,
                Lt => f < v,
                LtEqual => f <= v,
                Gt => f > v,
                GtEqual => f >= v,
                _ => false,
            }
        },
        _ => false,
    }
}

#[cfg(test)]
mod test {
    use message::matcher;
    use message::pb;

    fn add_field_integer(m: &mut pb::HekaMessage, name: &str, val: i64) {
        let mut f = pb::Field::new();
        f.set_value_type(pb::Field_INTEGER);
        f.set_name(name.into_string());
        f.add_value_integer(val);
        if val == 999 {
            f.add_value_integer(1024);
        }
        m.add_fields(f);
    }

    fn add_field_double(m: &mut pb::HekaMessage, name: &str, val: f64) {
        let mut f = pb::Field::new();
        f.set_value_type(pb::Field_DOUBLE);
        f.set_name(name.into_string());
        f.add_value_double(val);
        m.add_fields(f);
    }

    fn add_field_bool(m: &mut pb::HekaMessage, name: &str, val: bool) {
        let mut f = pb::Field::new();
        f.set_value_type(pb::Field_BOOL);
        f.set_name(name.into_string());
        f.add_value_bool(val);
        m.add_fields(f);
    }

    fn add_field_string(m: &mut pb::HekaMessage, name: &str, val: &str) {
        let mut f = pb::Field::new();
        f.set_value_type(pb::Field_STRING);
        f.set_name(name.into_string());
        f.add_value_string(val.into_string());
        m.add_fields(f);
    }

    fn add_field_bytes(m: &mut pb::HekaMessage, name: &str, val: Vec<u8>) {
        let mut f = pb::Field::new();
        f.set_value_type(pb::Field_STRING);
        f.set_name(name.into_string());
        f.add_value_bytes(val);
        m.add_fields(f);
    }

    fn get_test_message() -> pb::HekaMessage {
        let mut msg = pb::HekaMessage::new();
        // todo set uuid
        msg.set_timestamp(9000000000);
        msg.set_field_type("TEST".into_string());
        msg.set_logger("UnitTest".into_string());
        msg.set_severity(6);
        msg.set_payload("Test Payload".into_string());
        msg.set_env_version("0.8".into_string());
        msg.set_pid(1234);
        msg.set_logger("UnitTest".into_string());
        msg.set_hostname("example.com".into_string());
        add_field_string(&mut msg, "foo", "bar");
        add_field_integer(&mut msg, "number", 64);
        add_field_bytes(&mut msg, "data", vec!['d' as u8, 'a' as u8, 't' as u8, 'a' as u8]);
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
            Err(e) => fail!("{}", e.msg),
        };
        mm.is_match(m)
    }

    #[test]
    fn successful_creation() {
        assert!(matcher::Matcher::new("TRUE").is_ok());
        assert!(matcher::Matcher::new(" FALSE  ").is_ok());
        assert!(matcher::Matcher::new("Type == 'foo'").is_ok());
        assert!(matcher::Matcher::new("Type != 'foo'").is_ok());
        assert!(matcher::Matcher::new("Type >= 'foo'").is_ok());
        assert!(matcher::Matcher::new("Type <= 'foo'").is_ok());
        assert!(matcher::Matcher::new("Type > 'foo'").is_ok());
        assert!(matcher::Matcher::new("Type < 'foo'").is_ok());
        assert!(matcher::Matcher::new("Logger== 'foo'").is_ok());
        assert!(matcher::Matcher::new("Hostname =='foo'").is_ok());
        assert!(matcher::Matcher::new("EnvVersion=='foo'").is_ok());
        assert!(matcher::Matcher::new(r"Payload   ==  'foo\''").is_ok());
        assert!(matcher::Matcher::new("Uuid == 'foo'").is_ok());
        assert!(matcher::Matcher::new("Timestamp == 6").is_ok());
        assert!(matcher::Matcher::new("Severity == 6").is_ok());
        assert!(matcher::Matcher::new("Pid == 6").is_ok());
        assert!(matcher::Matcher::new("Pid != 6").is_ok());
        assert!(matcher::Matcher::new("Pid >= 6").is_ok());
        assert!(matcher::Matcher::new("Pid <= 6").is_ok());
        assert!(matcher::Matcher::new("Pid > 6").is_ok());
        assert!(matcher::Matcher::new("Pid < 6").is_ok());
        assert!(matcher::Matcher::new("Type =~ /^foo$/").is_ok());
        assert!(matcher::Matcher::new("Logger=~ /^foo$/").is_ok());
        assert!(matcher::Matcher::new("Hostname =~/^foo$/").is_ok());
        assert!(matcher::Matcher::new("EnvVersion=~/^foo$/").is_ok());
        assert!(matcher::Matcher::new("Payload   =~  /^foo$/").is_ok());
        assert!(matcher::Matcher::new("Uuid =~ /^foo$/").is_ok());
        assert!(matcher::Matcher::new("Fields[] == 0").is_ok());
        assert!(matcher::Matcher::new("Fields[foo] !~ /^foo$/").is_ok());
        assert!(matcher::Matcher::new("Fields[foo] == \"bar\"").is_ok());
        assert!(matcher::Matcher::new("Fields[foo] == NIL").is_ok());
        assert!(matcher::Matcher::new("Fields[foo] == TRUE").is_ok());
        assert!(matcher::Matcher::new("Fields[foo] == FALSE").is_ok());
        assert!(matcher::Matcher::new("Fields[0][0] == FALSE").is_ok());
        assert!(matcher::Matcher::new("Type == 'test' && (Severity==7 || Payload == 'Test Payload')").is_ok());
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
    fn successful_match() {
        let msg = get_test_message();
        assert!(test_match(&msg, "TRUE"));
        assert!(test_match(&msg, "(Severity == 7 || Payload == 'Test Payload') && Type == 'TEST'"));
        assert!(test_match(&msg, "EnvVersion == \"0.8\""));
        assert!(test_match(&msg, "EnvVersion == '0.8'"));
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
//      assert!(test_match(&msg, "Uuid == 'todo'"));
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

    #[test]
    fn failed_match() {
        let msg = get_test_message();
        assert!(!test_match(&msg, "FALSE"));
    }
}

