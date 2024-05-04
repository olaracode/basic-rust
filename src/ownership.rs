fn ownership_rules(){
    // ------ Ownership Rules ------
    // 1. Each value in rust has a variable that is called its owner
    // 2. There can only be one owner at a time
    // 3. When the owner goes out of scope, the value will be dropped.

    // This is handled on the memory stack since it's a fixed length|value
    { // S is not valid here or previously, it's not yet declared
        let s: &str = "Hello"; // s is valid from this point forward
        // We do stuff with s
    } // The scope is now over, and S is no longer valid
    // This is handled on the memory heap since it is a dynamic value
    { // Same rules apply here as they did previously
        let s: String = String::from("hello");
    }
}

fn main(){
    // This values are created on the stack,
    // Integers, Booleans and (? other type)
    // can be copied
    let x: i32 = 5;
    let y: i32 = x; // This is a copy of the x value

    // Since this values are created on the heap
    // Rust Invalidates s1 as soon as s2 is created
    // moving the pointer to s2 instead of both pointing to the same place
    let s1: String = String::from("Hello");
    // This means that if we try to use s1 after the 'value'
    // is 'moved' to s2 it will Pannic and throw an error
    let s2: String = s1; // Move (Not Shallow Copy)

    // if you instead want to create a copy of the value
    // without loosing the first reference you need to create a clone
    let s3: String = s2.clone();
    // Now we can access both s2 and s3 in the next line
    println!("{s2} {s2}");
}


fn ownership_in_functions(){
    // This value is owned by the ownership_in_functions fn
    let s: String = String::from("hello");
    let num: i32 = 255;
    // When we pass a parameter the receiving function takes ownership
    // Meaning the S is moved to the new function
    takes_ownership(s);
    // therefore we cannot access the s variable no longer
    // println!("{}", s); -> Throws an error

    // On the other hand when handling a variable that can be copied
    // it gets copied instead of moved. EG:
    makes_copy(num);
    println!("We can still access {}", num);

    // On the other hand, when a function returns a value
    // The ownership is moved to the scope of where the function gets executed
    let s1: String = gives_ownership();
    println!("s1 = {}", s1);

    // The ownership can be moved to a scope and then brought back
    // s1 belongs to this function scopes
    // as soon as it is passed the ownership is moved
    let s3: String = takes_and_gives_back(s1);
    // as soon as the value is returned to this scoped
    // the ownership moves as well
}

fn takes_ownership(some_string: String){
    // As soon as some_string is passed the ownership is moved to this scope
    println!("{}", some_string);
}

fn makes_copy(some_number: i32){
    // Since it receives a copy of the actual_number
    // Both functions have ownership of their respective variables
    println!("{}", some_number);
}

fn gives_ownership() -> String {
    // some_string Is created and owned by gives_ownership;
    let some_string: String = String::from("hello");
    // As soon as it is returned the ownership gets moved to the scope where
    // the funcion is calles
    some_string
}

fn takes_and_gives_back(a_string: String) -> String{
    // Takes ownership of the value when the function is called
    // And when the function ends it returns the ownership
    a_string
}
