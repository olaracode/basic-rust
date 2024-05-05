use std::collections::HashMap;

/*
Hash maps are used to store key: value pairs
*/
fn main() {
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(blue, 10);
    scores.insert(yellow, 50);
    println!("{:#?}", scores);

    // To get individual values from a hashmap
    let team_name = String::from("Blue");
    let blue_team_score = scores.get(&team_name);

    for (key, value) in &scores {
        println!("{}:{}", key, value);
    }

    /*
        While insert can add a new key: value pair to the hash map
        if the key already exists then it replaces the value

        In the case we want to add key:pair if it doesn't exist
        but do nothing if it does exist we can use hashmap.entry
    */

    // In this case since the Key: Red does not
    // exists so it adds it to the hashmap
    scores.entry(String::from("Red")).or_insert(35);

    // In this case since the Key: Red already exists it won't do anything
    scores.entry(String::from("Red")).or_insert(70);

    println!("{:#?}", scores);
    hash_from_value();
}

fn hash_from_value() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        // The or_insert method returns a mutable reference of the
        // value
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
