/*
    References Rules:
    1. At any given time you can have either:
        One mutable reference
        Or
        N number of immutable references
    2. References must always be valid.
*/

fn main() {
    /**
        Here we create a variable which we'll use
        Since its a string the ownership gets moved into
        the calculate_length_with_ownership function
        Since we don't want to lose said variable
        we return both the length and the variable itself
        Making so we bounce the variable into a function and then back
        Into scope, which is counter-intuitive
    */
    let s1: String = String::from("Hello");
    let (s2, len) = calculate_length_with_ownership(s1);
    println!("The length of '{}' is {}.", s2, len);

    /*
        To fix this we'll use a reference
        a reference is when you borrow a value without
        taking ownership of said value
        to create a reference we have to use '&var_name'
    */
    let s3: String = String::from("World");
    let s3_length = calculate_length(&s3);

    /*
        Since references are not mutable values
        to modify a value without taking direct ownership
        we need to:
        1. Create a mutable variable using the mut keyword
        2. Pass a mutable reference to the function using
        &mut variable_name

        Mutable references have a limitation.
        We can only use ONE mutable reference
        for a variable

        You also can't use a mutable reference alongside a unmutable reference
        meaning you either have
    */
    let mut s4: String = String::from("Hello");
    change(&mut s4);
}

fn calculate_length_with_ownership(s: String) -> (String, usize){
    let length: usize = s.len();
    (s, length)
}

/*
    Since this funcion is going to borrow a value
    We declare the parameter as a reference
    using '&data_type' notation.
    References are non-mutable values
    meaning you cannot update a borrowed value
*/
fn calculate_length(s: &String) -> usize {
    let length = s.len();
    length
}

/*
    To be able to modify|update a reference in a function
    we need to specify that it is going to be a mutable reference
    using &mut DataType
*/
fn change(some_string: &mut String){
    some_string.push_str(", World");
}
