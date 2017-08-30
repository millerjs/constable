use datatypes::DataType;

#[derive(Debug, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub datatype: DataType,
}
