use std::fs::File;
use std::io::ErrorKind;
/*
    std::fs::File -> allows to handle files
*/
fn main() {
    /*
        The result enum works for handling errors like the Option enum
        Giving a OK or an Error
        enum Result<T, E> {
            Ok(T),
            Err(E)
        }
    */

    let file = File::open("hello.txt");

    /*
        This way of error handling
        uses the match expression along the ErrorKind
        to gracefully handle the error.
        If the file is not found(Meaning an error was thrown)
        Then we create the file, But file creation can also fail
        So we again use a match expression to see if the creation was
        successful
    */
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem Creating the file {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file {:?}", other_error)
            }
        },
    };
    /*
        This is the same operation as before.
        Attempt to get the file, if it doesn't exist then we attempt to create it
        But in this instance we are using the unwrap_or_else method
    */
    let file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt")
                .unwrap_or_else(|error| panic!("Problem creating the file {:?}", error))
        } else {
            panic!("Problem opening the file {:?}", error)
        }
    });
}
