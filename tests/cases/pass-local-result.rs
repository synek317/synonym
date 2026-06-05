// https://github.com/synek317/synonym/issues/11
// test case author: https://github.com/mlhetland

#![allow(dead_code)]

use synonym::Synonym;

enum Error {
    Foo,
    Bar,
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Synonym)]
struct Baz(u32);

fn main() {}
