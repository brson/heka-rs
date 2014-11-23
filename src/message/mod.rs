pub mod pb; // add generated file to the project
pub mod matcher;

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

pub fn get_field_string<'a>(msg: &'a pb::HekaMessage, name: &str, fi: uint, ai: uint) -> Option<&'a String> {
    match find_field(msg, name, fi as uint)
    {
        Some(f) => {
            let a = f.get_value_string();
            let l = a.len();
            if ai < l {
                Some(&a[ai])
            } else {
                None
            }
        },
        None => None,
    }
}

pub fn get_field_bool<'a>(msg: &'a pb::HekaMessage, name: &str, fi: uint, ai: uint) -> Option<bool> {
    match find_field(msg, name, fi as uint)
    {
        Some(f) => {
            let a = f.get_value_bool();
            let l = a.len();
            if ai < l {
                Some(a[ai])
            } else {
                None
            }
        },
        None => None,
    }
}

pub fn get_field_number<'a>(msg: &'a pb::HekaMessage, name: &str, fi: uint, ai: uint) -> Option<f64> {
    match find_field(msg, name, fi as uint)
    {
        Some(f) => {
            match f.get_value_type() {
                pb::Field_ValueType::INTEGER => {
                    let a = f.get_value_integer();
                    let l = a.len();
                    if ai < l {
                        Some(a[ai] as f64)
                    } else {
                        None
                    }
                },
                pb::Field_ValueType::DOUBLE => {
                    let a = f.get_value_double();
                    let l = a.len();
                    if ai < l {
                        Some(a[ai] as f64)
                    } else {
                        None
                    }
                },
                _ => None,
            }
        },
        None => None,
    }
}
