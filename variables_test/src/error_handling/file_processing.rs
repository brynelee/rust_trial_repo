use std::fs::File;
use std::io::ErrorKind;

fn main() {
    match File::open("hello.txt") {
        Ok(_) => println!("open success"),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(_) => println!("create success"),
                Err(e) => println!("create fail {:#?}", e),
            },
            other_err => println!("open fail {:#?}", other_err),
        },
    }
}
