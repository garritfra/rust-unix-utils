use std::env;
use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if !args.is_empty() {
        return concat_files(args);
    }

    Ok(())
}

fn concat_files(paths: Vec<String>) -> io::Result<()> {
    for path in paths {
        let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", path, why),
            Ok(file) => file,
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(_) => {
                continue;
            }
            Ok(_) => {
                print!("{}", s);
            }
        }
    }

    Ok(())
}
