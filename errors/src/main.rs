use std::fs::File;
use std::io::Read;

fn main() {
    // let f = match File::open("hello.txt") {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(error2) => match error2.kind() {
    //                 ErrorKind::PermissionDenied => {
    //                     println!("PermissionDenied. Could not create file")
    //                 }
    //                 other_error => panic!("Problem creating the file: {:?},e"),
    //             },
    //         },
    //         other_error => panic!("Unkown problem creating file {:?}", error),
    //     },
    // };

    let f = read_file("Hello.txt".to_string());
}

fn read_file(filename: String) -> Result<String, std::io::Error> {
    let mut s = String::new();
    File::open(filename)?.read_to_string(&mut s)?;
    Ok(s)
}
