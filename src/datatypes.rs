//! Core Constable data types

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub enum DataType {
    Text,
    BigInteger,
    Integer,
    DateTime,
    Boolean,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub enum DataValue {
    Text(Option<String>),
    BigInteger(Option<i64>),
    Integer(Option<i32>),
    DateTime(Option<String>),
    Boolean(Option<bool>),
}
