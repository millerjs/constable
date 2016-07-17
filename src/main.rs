//! Constable Database

#[macro_use]
extern crate quick_error;
extern crate bincode;
extern crate rustc_serialize;

use std::io;
use std::fs::OpenOptions;
use std::io::prelude::*;
use bincode::rustc_serialize::{encode, decode, decode_from};

quick_error! {
    #[derive(Debug)]
    pub enum ConstableError {
        /// IO Error
        Io(err: std::io::Error) { from() }
        /// Encoding Error
        Encoding(err: bincode::rustc_serialize::EncodingError) { from() }
        /// Decoding Error
        Decoding(err: bincode::rustc_serialize::DecodingError) { from() }
    }
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub enum DataType {
    Text,
    BigInteger,
    Integer,
    DateTime,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub enum DataValue {
    Text(Option<String>),
    BigInteger(Option<i64>),
    Integer(Option<i32>),
    DateTime(Option<String>),
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Table {
    pub name: String,
    current_id_seq: u64,
    columns: Vec<Column>,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Column {
    pub name: String,
    pub datatype: DataType,
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Row {
    pub id: u64,
    pub deleted: bool,
    pub version: u64,
    pub columns: Vec<DataValue>,
}

pub struct TableScanner<R> {
    pub cursor: u64,
    pub reader: R,
}

pub type ConstableResult<T> = Result<T, ConstableError>;

impl Row {
    fn new(columns: Vec<DataValue>) -> Row
    {
        Row { id: 0, version: 0, deleted: false, columns: columns }
    }
}

impl Table {
    fn new<S>(name: S) -> Table
        where S: Into<String>
    {
        Table {
            name: name.into(),
            columns: vec![],
            current_id_seq: 0,
        }
    }

    fn init<R>(mut self, reader: &mut R) -> ConstableResult<Table>
        where R: Read + Seek
    {
        self.current_id_seq = self.scan(reader).fold(
            0, |acc, row| std::cmp::max(acc, row.unwrap().id));
        Ok(self)
    }

    fn column<S>(mut self, name: S, datatype: DataType) -> Table
        where S: Into<String>
    {
        self.columns.push(Column {
            name: name.into(),
            datatype: datatype,
        });
        self
    }

    fn insert<W>(&self, writer: &mut W, row: Row) -> ConstableResult<()>
        where W: Write
    {
        let bytes = try!(encode(&row, bincode::SizeLimit::Infinite));
        let length = try!(encode(&bytes.len(), bincode::SizeLimit::Infinite));
        try!(writer.write(&length));
        try!(writer.write(&bytes));
        Ok(())
    }

    fn scan<R>(&self, mut reader: R) -> TableScanner<R>
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
            Err(error) => return None
        };
        match decode_from(&mut self.reader, bincode::SizeLimit::Bounded(length)) {
            Ok(row) => Some(Ok(row)),
            Err(error) => Some(Err(ConstableError::from(error)))
        }
    }
}

fn main() {
    let mut posts_file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .append(true)
        .open("posts.dat")
        .unwrap();

    let mut posts_table = Table::new("posts")
        .column("username", DataType::Text)
        .column("body", DataType::Text)
        .init(&mut posts_file)
        .unwrap();

    println!("Created table: \n{:?}", posts_table);

    posts_table.insert(&mut posts_file, Row::new(vec![
        DataValue::Text(Some("user1".to_string())),
        DataValue::Text(Some("This is a blog".to_string())),
    ]));

    for row in posts_table.scan(&mut posts_file) {
        println!("{:?}", row);
    }
}
