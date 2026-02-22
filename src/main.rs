use std::fs;
use std::process::exit;
fn main() {
    let filepath = "docs.gl/gl4/glClear.xhtml";
    let content = fs::read_to_string(filepath).unwrap_or_else(|err| {
        eprint!("Error : could not open the file {filepath}:{err}");
        exit(1);
    });
    println!("Length of {filepath} is {length}", length = content.len());
}
