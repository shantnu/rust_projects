use std::fs;

fn main() {
    let files = fs::read_dir("./").expect("failed to read dir");

    for f in files {

        let local_path = f.expect("failed to open file");
        if local_path.path().is_dir() {
            println!("{} is a direcrtory!", local_path.path().display());
        } else {
            println!("{} NOT  a direcrtory!", local_path.path().display());
        }
        // println!("{}", f.expect("failed to open file").path().display());
    }
}
