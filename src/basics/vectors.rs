// mod guessing_num;
/*
    Collection Types

    Vector: is similar to an array but they can
    have dynamic size
*/
#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
fn main() {
    let a = [1, 2, 3];

    // We have to set the type since
    // we are starting an empty Vector
    let mut v: Vec<i32> = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);

    // Here we initialize the vector with values
    // and we don't have to add declarative typing
    let v2 = vec![1, 2, 3];
    println!("The third element is {}", v[2]);

    /*
        If you want to access a index that
        may or may not exist and don't want
        the app to panick|crash

        You can use the get method
    */
    match v.get(3) {
        Some(third) => println!("The fourth element is {}", third),
        None => println!("There is no fourth element"),
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];

    match &row[1] {
        SpreadSheetCell::Int(i) => println!("{}", i),
        // We use _ to signify every other value that is not the
        // One we expect
        _ => println!("Not an integer!"),
    };

    println!("{:#?}", row);
}
