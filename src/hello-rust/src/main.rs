use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let out = b"Hello fellow Rustaceans!";
    let width = out.len();

    let mut writer = BufWriter::new(stdout());
    say(out, width, &mut writer).unwrap();
}