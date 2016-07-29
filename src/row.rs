use datatypes::DataValue;

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Row {
    pub id: u64,
    pub deleted: bool,
    pub version: u64,
    pub columns: Vec<DataValue>,
}

impl Row {
    pub fn new(columns: Vec<DataValue>) -> Row
    {
        Row { id: 0, version: 0, deleted: false, columns: columns }
    }
}
