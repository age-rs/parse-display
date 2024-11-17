pub trait RegexForFromStr: core::str::FromStr {
    fn regex_for_from_str() -> String;
}
impl RegexForFromStr for String {
    fn regex_for_from_str() -> String {
        "(?s:.*?)".into()
    }
}

impl RegexForFromStr for bool {
    fn regex_for_from_str() -> String {
        r"true|false".into()
    }
}

fn regex_uint() -> String {
    r"[0-9]+".into()
}
impl RegexForFromStr for u8 {
    fn regex_for_from_str() -> String {
        regex_uint()
    }
}
impl RegexForFromStr for u16 {
    fn regex_for_from_str() -> String {
        regex_uint()
    }
}
impl RegexForFromStr for u32 {
    fn regex_for_from_str() -> String {
        regex_uint()
    }
}
impl RegexForFromStr for u64 {
    fn regex_for_from_str() -> String {
        regex_uint()
    }
}
impl RegexForFromStr for u128 {
    fn regex_for_from_str() -> String {
        regex_uint()
    }
}

fn regex_sint() -> String {
    r"-?[0-9]+".into()
}
impl RegexForFromStr for i8 {
    fn regex_for_from_str() -> String {
        regex_sint()
    }
}
impl RegexForFromStr for i16 {
    fn regex_for_from_str() -> String {
        regex_sint()
    }
}
impl RegexForFromStr for i32 {
    fn regex_for_from_str() -> String {
        regex_sint()
    }
}
impl RegexForFromStr for i64 {
    fn regex_for_from_str() -> String {
        regex_sint()
    }
}
impl RegexForFromStr for i128 {
    fn regex_for_from_str() -> String {
        regex_sint()
    }
}
