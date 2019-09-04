use std::io;
use std::io::Read;
use std::fs::File;

fn read_sth_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn main(){
    let result = read_sth_from_file();
    println!("the content of the file is {:?}", result);
}
