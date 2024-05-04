/*
    Structs allow you to group related data together,
    kind of like an object attributes in OOP Languages

    In this example we will create a struct for User data
*/

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn main(){
    let mut user1 = User {
        email: String::from("octavioasapchi@gmail.com"),
        username: String::from("olaracode"),
        active: true,
        sign_in_count: 1
    };

    let name = user1.username;
    user1.username = String::from("olarav1");

    let user2 = build_user(
        String::from("something@mail.com"),
        String::from("something")
    );

    let user3 = User {
        email: String::from("john@doe.com"),
        username: String::from("JohnDoe"),
        ..user2 // similar to spread operator to fill the remaining values
    };

    /*
        You can also create a "Tuple Struct"
        where you don't have any named fields
    */

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
