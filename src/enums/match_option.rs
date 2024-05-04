fn main(){
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn if_let_syntax(){
    let some_value = Some(3);
    if let Some(3) = some_value {
        println!("three");
    }
}
