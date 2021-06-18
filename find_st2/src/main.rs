use std::env;
use walkdir::WalkDir;

fn find_st2(string_to_find: String) -> Vec<String> {
    let mut v:Vec<String> = Vec::new();
    for entry in WalkDir::new(".") {
        let entry = entry.unwrap();
        //println!("{}", entry.path().display());

        let dir_found = entry.file_name().to_str().unwrap().to_lowercase();
        if dir_found == string_to_find {

            v.push(entry.path().display().to_string());
        }
    }

    v
}
fn main() {
    let args: Vec<String> = env::args().collect();

    let string_to_find = args[1].to_string();

    let v = find_st2(string_to_find);


    println!("Found files: {:?}", v);
}

#[cfg(test)]

mod tests {

    #[test]
    fn test_find_st() {
        use super::find_st2;
        let string_to_find = "foobar.txt".to_string();

        let v = find_st2(string_to_find);

        assert_eq!(v, ["./temp/temp/temp/foobar.txt", "./temp/fooBAR.txt", "./foobar.txt"])
    }
}
