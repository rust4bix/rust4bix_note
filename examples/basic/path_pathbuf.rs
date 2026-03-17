use std::path::{Path, PathBuf};


fn print_path(path: &Path) {
    println!("{}", path.display());
}

fn main() {
    let mut buf = PathBuf::from("data");
    buf.push("file.txt");
    print_path(&buf);
}
