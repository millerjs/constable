//! Constable Database

extern crate constable;

use constable::table::Table;
use constable::row::Row;
use constable::datatypes::{DataType, DataValue};

use std::fs::OpenOptions;

fn main() {
    let mut posts_file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .append(true)
        .open("posts.dat")
        .unwrap();

    let posts_table = Table::new("posts")
        .column("username", DataType::Text)
        .column("body", DataType::Text)
        .column("active", DataType::Boolean)
        .init(&mut posts_file)
        .unwrap();

    println!("Created table: \n{:?}", posts_table);

    let _ = posts_table.insert(&mut posts_file, Row::new(vec![
        DataValue::Text(Some("user1".to_string())),
        DataValue::Text(Some("This is a blog".to_string())),
        DataValue::Boolean(Some(false)),
    ]));

    for row in posts_table.scan(&mut posts_file) {
        println!("{:?}", row);
    }
}
