// created by Abhilekh Gautam jul 22, 2023 22:30

use std::env;

pub mod document;
pub mod document_viewer;

use document::Document;
use document_viewer as viewer;

use viewer::Viewer;

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
    let doc = Document::new(file_path);

    // Create a Document Viewer
    let viewer = Viewer::new(doc);

    viewer.view();
}
