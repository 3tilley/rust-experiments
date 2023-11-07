use std::fs::File;
use std::path::Path;

fn main() {
    let path = Path::new(file!()).parent().unwrap();

    let f = fs_err::File::open(path);
    let j = serde_json::from_reader(f).unwrap();
    println!("{:?}", f);

    let f_vanilla = File::open(path);
    let j_vanilla = serde_json::from_reader(f_vanilla).unwrap();
    println!("{:?}", j_vanilla);
}
