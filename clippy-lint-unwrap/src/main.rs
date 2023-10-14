fn main() {
    println!("Hello, world!");
    let path = Path::new(file!()).parent().unwrap();
    let f = fs_err::File::open(path);
    let j = serde_json(f).unwrap();
    println!("{:?}", f)
}
