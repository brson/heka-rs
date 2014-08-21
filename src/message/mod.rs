pub mod pb; // add generated file to the project

pub fn find_field<'a>(msg: &'a pb::HekaMessage, name: &str, fi: uint) -> Option<&'a pb::Field> {
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

pub fn match_field<'a>(msg: &'a pb::HekaMessage, name: &str, fi: uint, ai: uint, val: &str) -> bool {
    match find_field(msg, name, fi as uint)
    {
        Some(f) => {
            let a = f.get_value_string();
            let l = a.len();
            if ai < l &&  a[ai].as_slice() == val {
                true
            } else {
                false
            }
        },
        None => {false}
    }
}

pub fn match_field_numeric<'a>(msg: &'a pb::HekaMessage, name: &str, fi: uint, ai: uint, val: f64) -> bool {
    match find_field(msg, name, fi as uint)
    {
        Some(f) => {
            match f.get_value_type() {
                pb::Field_INTEGER => {
                    let a = f.get_value_integer();
                    let l = a.len();
                    if ai < l &&  a[ai] == val as i64 {
                        true
                    } else {
                        false
                    }
                },
                pb::Field_DOUBLE => {
                    let a = f.get_value_double();
                    let l = a.len();
                    if ai < l &&  a[ai] == val {
                        true
                    } else {
                        false
                    }
                },
                _ => false
            }
        },
        None => false
    }
}
