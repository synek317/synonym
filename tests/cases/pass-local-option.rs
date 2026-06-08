#![allow(dead_code)]

use synonym::Synonym;

struct Option;

#[derive(Synonym)]
struct LocalOption(u32);

fn main() {
    let _ = LocalOption(1).partial_cmp(&LocalOption(2));
}
