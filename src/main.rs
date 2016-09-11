extern crate json;

use std::io::{self, Read, Write};


fn beautify_json(ugly: &str) -> String {
    let parsed = json::parse(ugly).unwrap();
    json::stringify_pretty(parsed, 4)
}

fn main() {
    let mut raw = String::new();
    io::stdin().read_to_string(&mut raw).unwrap();
    io::stdout().write(beautify_json(&raw).as_bytes()).unwrap();
}
