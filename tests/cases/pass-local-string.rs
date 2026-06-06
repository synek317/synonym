#![allow(dead_code)]

use synonym::Synonym;

struct String;

#[derive(Synonym)]
struct LocalString(Box<str>);

fn main() {
    let _ = LocalString::from("abc".to_string());
}
