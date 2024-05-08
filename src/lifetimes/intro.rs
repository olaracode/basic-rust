/*
    Lifetimes are how rust handle where a variable ends its cycle,
    meaning it gets dropped out of memory.

    One example would be a scoped variable
    fn main(){
        let y: i32; -> Y Is created
        {
            let x = 5; -> X is created
            y = &x; --> Y Borrows the value of X
        } -> X is dropped
        println!("{}", y); -> Compiler error because Y points
        --> to a unexisting reference
    }
*/

fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
}

/*
This code will show an error given that the return
lifetime is not defined. X or Y could have different lifetime cycles
meaning that the return as well could have different lifetime cycles
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
*/

/*
Lifetime anotation
&i32 -----------> A reference
&'a i32 --------> A reference with an explicit lifetime
&'a mut i32 ----> A mutable reference with an explicit lifetime

lifetime annotations don't change the lifetime of a value
They just tie together a lifetime for different values|Variables
*/

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
