use std::fs::File;
use std::io::ErrorKind;
/*
    std::fs::File -> allows to handle files
*/
fn main() {
    /*
        Here we use the unwrap method which will return the value if possible
        or panic!()
    */
    let f = File::open("hello.txt").unwrap();

    /*
        On the other hand if we want to pass a message we can use the expect
        Which works for expected errors and we can pass a custom message
    */
    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem Opening the file: {:?}", error),
    // };
}
