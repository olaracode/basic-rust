use std::error::Error;
use std::fs::{self, File};
use std::io;
use std::io::Read;
/*
    std::fs::File -> allows to handle files
*/

/*
    ? Error Propagation
    Error propagation allows us to "throw" an error in a scope
    and return said error to the scope where it should be used

    For example this read_username_from_file allows us to move the content
    from a external file(hello.txt) into a variable
    We don't necesarily want the code to panic! Because the placer where it
    is being used maybe knows how to handle that scenario, so we'd rather
    allow the caller decide what to do if there is an error

    To do this we use match expressions to see if the operations work,
    if they do work then we continue, if they don't we return a new Error
    created using Err(error: Error)
*/

/*
    ! The "?" operator
    The ? operator allows us to make use of an early return
    Meaning that in this expression:
    let f = File::open("hello.txt")?;

    If the file is found then it returns(sort of) the file and stores it in f
    But if it isn't found it will finish the function returning the error.

    Meaning that
    let f = File::open("hello.txt")?;

    And
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    Do exactly the same.

    ! NOTES:
    ! It can only be used in functions that return an Option<> or a Result<>
*/

// Example using Match and Err
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// Example using ? operator
fn read_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// Chained method calls

fn chained_method_calls() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_to_string() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

/*
  ! Since the question mark operator can only be used in a function that returns
  ! an option or a result we can do this to allow it's usage in the main fn
*/
fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    // This is required to Comply with the Response type of main
    Ok(())
}
