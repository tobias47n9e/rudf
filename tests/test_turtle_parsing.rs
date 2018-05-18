extern crate rudf;

use rudf::model;
use rudf::rio::turtle;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::fs::read_dir;

/// Test all the files in the turtle_test_data directory
#[test]
fn test_simple_turtle_parsing() {
    let data_factory = model::data::DataFactory::default();
    let paths = fs::read_dir("tests/turtle_test_data").expect("No");

    for entry in paths {
        let path = entry.unwrap().path();
        let file_name = path.file_name().unwrap();
        let file_name_as_str = file_name.to_str().unwrap();
        let file_read = File::open(
            format!("tests/turtle_test_data/{}", file_name_as_str)).unwrap();
        turtle::read_turtle(file_read, &data_factory);
    }
}
