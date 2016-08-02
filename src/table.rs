use std::io;
use std::io::prelude::*;

use std;
use result::ConstableResult;
use bincode;
use bincode::rustc_serialize::{encode, decode_from};
use column::Column;
use row::Row;
use datatypes::DataType;
use errors::ConstableError;


#[derive(Debug)]
pub struct Table {
    pub name: String,
    current_id_seq: u64,
    columns: Vec<Column>,
}

pub struct TableScanner<R> {
    pub cursor: u64,
    pub reader: R,
}


impl Table {
    pub fn new<S>(name: S) -> Table
        where S: Into<String>
    {
        Table {
            name: name.into(),
            columns: vec![],
            current_id_seq: 0,
        }
    }

    pub fn init<R>(mut self, reader: &mut R) -> ConstableResult<Table>
        where R: Read + Seek
    {
        self.current_id_seq = self.scan(reader).fold(
            0, |acc, row: ConstableResult<Row>| std::cmp::max(acc, row.unwrap().id));
        Ok(self)
    }

    pub fn column<S>(mut self, name: S, datatype: DataType) -> Table
        where S: Into<String>
    {
        self.columns.push(Column {
            name: name.into(),
            datatype: datatype,
        });
        self
    }

    pub fn insert<W>(&self, writer: &mut W, row: Row) -> ConstableResult<()>
        where W: Write
    {
        let bytes = try!(encode(&row, bincode::SizeLimit::Infinite));
        let length = try!(encode(&bytes.len(), bincode::SizeLimit::Infinite));
        try!(writer.write(&length));
        try!(writer.write(&bytes));
        try!(writer.flush());
        Ok(())
    }

    pub fn scan<R>(&self, mut reader: R) -> TableScanner<R>
        where R: Read + Seek
    {
        reader.seek(io::SeekFrom::Start(0));
        TableScanner {
            cursor: 0,
            reader: reader,
        }
    }
}

impl<R: Read + Seek> Iterator for TableScanner<R> {
    type Item = ConstableResult<Row>;

    fn next(&mut self) -> Option<ConstableResult<Row>> {
        let length = match decode_from(&mut self.reader, bincode::SizeLimit::Infinite) {
            Ok(len) => len,
            Err(_) => return None
        };
        match decode_from(&mut self.reader, bincode::SizeLimit::Bounded(length)) {
            Ok(row) => Some(Ok(row)),
            Err(error) => Some(Err(ConstableError::from(error)))
        }
    }
}
