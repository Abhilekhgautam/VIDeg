// created by Abhilekh Gautam jul 22, 2023 22:30

use std::env;
use std::fs;

// Represents the overall Document

pub struct Document<'a> {
    // oops
    file_name: &'a str,
    content: String,
}

impl<'a> Document<'a> {
    fn new(file_path: &'a str) -> Self {
        let contents = fs::read_to_string(file_path).unwrap();
        Self {
            // todo: stripe the path to only contain the file name
            file_name: file_path,
            content: contents,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Invalid Argument list");
        std::process::exit(-1);
    }

    let file_path = &args[1];

    if !std::path::Path::new(file_path).exists() {
        println!("{file_path}: not a File or Directory");
        std::process::exit(-1);
    }

    // create a new document
    let _doc = Document::new(file_path);

    // todo:
    // Create a Document Viewer
    // let viewer = new Viewer(doc);
}
