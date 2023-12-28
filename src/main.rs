use std::{fs::File, io::Read};

mod dom;
mod html;

fn main() {
    let html_input = std::env::args().nth(1).expect("please provide a file name");
    let html = read(html_input);
    let root = html::parse(html);

    println!("{:#?}", root);
}

fn read(file: String) -> String {
    let mut input = String::new();
    File::open(file)
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    input
}
