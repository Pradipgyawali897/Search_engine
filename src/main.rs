use core::panic;
#[warn(unused)]
use std::fs::{self, ReadDir};
use std::{fs::File, io};
use xml::reader::{EventReader, XmlEvent};

fn read_entire_xml_file(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let er = EventReader::new(file);
    let mut content = String::new();

    for event in er.into_iter() {
        if let XmlEvent::Characters(text) = event? {
            content.push_str(&text);
        }
    }

    Ok(content)
}

fn main() {
    let filepath = "docs.gl/gl4/glClear.xhtml";
    let fir_path = "docs.gl/gl4";
    let dir = fs::read_dir(fir_path).unwrap_or_else(|err| {
        panic!("Error with reading the file :{err}");
    });

    for file in dir {
        println!("{file:?}");
    }
    /*println!(
        "{} is the content ",
        read_entire_xml_file(filepath).expect("Todo")
    );*/
}
