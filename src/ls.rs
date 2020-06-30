use std::fs;

fn main() {
    if let Ok(current_dir) = fs::read_dir(".") {
        for path in current_dir {
            println!("{}", path.unwrap().path().display());
        }
    }
}
