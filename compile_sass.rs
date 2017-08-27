extern crate sass_rs;
use sass_rs::*;

use std::fs::File;
use std::io::Write;

fn main() {
    let mut options = Options::default();
    options.output_style = OutputStyle::Compressed;

    let mut f = File::create("static/css/dark-main.css").unwrap();
    f.write_all(compile_file("static/scss/dark-main.scss", options).unwrap().as_bytes()).unwrap();
}
