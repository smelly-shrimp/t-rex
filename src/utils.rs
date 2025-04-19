use std::{fs, path::Path};

pub fn fill_path(path_str: &String) {
    let path = Path::new(&path_str);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
}
