#![feature(proc_macro_hygiene)]

use include_dir::include_dir;

const PARENT_DIR: include_dir::Dir<'_> = include_dir!("src");
fn main() {
    println!("{:#?}", PARENT_DIR);
}
