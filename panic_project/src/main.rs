use std::fs::File;
use std::io::Error;
use std::io::Read;

fn main() {
    // println!("Hello, world!");
    // panic!("crash and burn")

    // let v = vec![1, 2, 3];
    // v[99];
    // let f = File::open("Hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(err) => {
    //         panic!("Error opening file {:?}", err)
    //     }
    // };
    // let f = match f {
    //     Ok(file) => file,
    //     Err(err) => match err.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Error create file {:?}", e),
    //         },
    //         oe => panic!("Error create file {:?}", err),
    //     },
    // };
    let f = File::open("Hello.txt").unwrap();

    read_username_from_file();
}

fn read_username_from_file() -> Result<String, Error> {
    let mut f = File::open("Hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
