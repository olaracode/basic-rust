/*
    In Rust there are no Null Values
    To handle this the Option Enum is included into the language
    It looks like this:

    enum Option<T> {
        Some(T),
        None
    }

    Where 'T' is a generic type, meaning it will use the value
    of the variable|data passed or the explicit type annotated by
    us when no data is passed on initialization
*/

fn use_option(){
    let x: i32 = 8;
    let y: Option<i8> = Some(5);

    let absent_number: Option<i32> = None;

    // let result = x + y;
    // This won't work since Y is not guaranteed to exist
    // Given that it is an Option<i8> type
    // To make it work we can use the 'unwrap_or' method
    let result = x + y.unwrap_or(0);
}
