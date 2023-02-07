use std::env;
use std::fs;

pub struct ParsedCask {
    version: String,
    sha256: String,
    url: String,
    name: String,
    desc: String,
    homepage: String
}

fn parse_cask_file(path: String) {
    let file_contents = fs::read_to_string(path)
        .expect("Unable to read cask file");

    let options = lib_ruby_parser::ParserOptions {
        buffer_name: "(eval)".to_string(),
        ..Default::default()
    };

    let mut parser = lib_ruby_parser::Parser::new(file_contents.as_bytes(), options);

    println!("{:?}", parser.do_parse());
}