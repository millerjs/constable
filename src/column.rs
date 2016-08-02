use datatypes::DataType;

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Column {
    pub name: String,
    pub datatype: DataType,
}
