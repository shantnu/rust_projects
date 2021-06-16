use std::fs;

fn main() {
    println!("Hello, world!");

    let files = fs::read_dir("./").expect("failed to read dir");

    for f in files {
        println!("{}",f.expect("failed to open file").path().display());
    }
}
