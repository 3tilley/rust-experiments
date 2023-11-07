use std::path::Path;

struct File {
    path: String,
}

impl File {
    fn new(path: &Path) -> Self {
        Self {
            path: path.to_str().unwrap().to_string(),
        }
    }
}

fn open(path: &Path) -> Option<File> {
    Some(File::new(&path))
}

trait FileAPI {
    fn read(self) -> String;
}

impl FileAPI for File {
    fn read(self) -> String {
        std::fs::read_to_string(&self.path).unwrap()
    }
}

fn test_file<T>(f: T) -> String
where
    T: FileAPI,
{
    f.read()
}

fn main() {
    // let suggest_path: &Path = Path::new(file!()).parent();
    // let suggest_parent: &Path = suggest_path.parent();

    // let suggest_int: u32 = "2".try_into();

    let path = Path::new(file!()).parent().unwrap();
    let file = open(path);
    let _ = test_file(file);
}
