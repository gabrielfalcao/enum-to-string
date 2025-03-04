pub trait EnumToString {
    fn as_str(&self) -> &'static str;
    fn as_dbg(&self) -> &'static str;
}

pub trait SerdeEnumToString {
    fn as_str(&self) -> &'static str;
    fn as_dbg(&self) -> &'static str;
}
