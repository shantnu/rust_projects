use std::fs;
use std::env;

fn search_sub_dir(local_path: &std::fs::DirEntry, string_to_find: &String,  v: &mut Vec<String>)  {
    let subdir = fs::read_dir(local_path.path()).expect("Path dont exist bro");

    for subd in subdir {
        let subd_path = subd.unwrap();
        //println!("{}", subd_path.path().display()); //fix
        //println!("{}", subd_path.path().display().to_string().to_lowercase()); //fix
        let abc = subd_path.file_name();
        //println!("File name was {:?}", abc.clone().into_string().unwrap().to_lowercase() );

        // clone needed-- why? ownership moved? check why.
        // also, why unwrap needed in left side? I think to convert from <result> to string by forcing it to get success case,
        //igrnoing <Error>
        if abc.into_string().unwrap().to_lowercase() == string_to_find.clone() {
            println!("{}", subd_path.path().display().to_string());
            v.push(subd_path.path().display().to_string());
        }

    }

}
fn find_st(string_to_find: String) -> Vec<String> {

    // can explain why mut needed when do push
    let mut v:Vec<String> = Vec::new();
    let files = fs::read_dir("./").expect("failed to read dir");
    //let string_to_find = "foorbar.txt".to_string();

    for f in files {
        // have to copy to take ownership
        let local_path = f.expect("failed to open file");
        let abc = local_path.file_name();
        //println!("File name was {:?}", abc.clone().into_string().unwrap().to_lowercase() );

        // clone needed-- why? ownership moved? check why.
        // also, why unwrap needed in left side? I think to convert from <result> to string by forcing it to get success case,
        //igrnoing <Error>
        if abc.into_string().unwrap().to_lowercase() == string_to_find.clone() {
            v.push(local_path.path().display().to_string());
            println!("{}", local_path.path().display().to_string());


        }

        if local_path.path().is_dir() {
            //println!("{} is a direcrtory!", local_path.path().display());
            search_sub_dir(&local_path, &string_to_find, &mut v);


        } else {
            //println!("{} NOT  a direcrtory!", local_path.path().display());
        }

        // println!("{}", f.expect("failed to open file").path().display());
    }

    // just v! explain
    v

}
fn main() {
    let args: Vec<String> = env::args().collect();

    let string_to_find = args[1].to_string();

    let v = find_st(string_to_find);

    println!("Found: {:?}", v); // explain whta :? is

}


#[cfg(test)]

mod tests {

    #[test]
    fn test_find_st() {
        use super::find_st;
        let string_to_find = "foobar.txt".to_string();

        let v = find_st(string_to_find);

        assert_eq!(v,  ["./temp/fooBAR.txt", "./foobar.txt"])
    }
}