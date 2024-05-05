use unicode_segmentation::UnicodeSegmentation;

fn main() {
    // Strings are stored as a collection of UTF-8 encoded bytes
    // ASCII -> American Standard Code For information interchange
    // In ASCII all characters are represented by 1 bit
    // In UTF-8 Encoding characters can be 1, 2, 3 or 4 bits

    // Ways to create new Strings
    let s1 = String::new(); // Using the new() method
    let s2 = "Initial Contents"; // From a String slice
    let s3 = s2.to_string(); // Converting a string slice(&str) -> String
    let s4 = String::from("Initial contents"); // using from(&str) method

    // Just like a vector you can append | extend a string

    let mut s = String::from("foo");
    s.push_str("bar"); // push a string into another string
    s.push('!'); // push a character into a string
    println!("{}", s);

    /*
        Since strings are encoded bytes
        You can't index them like you would an Array | Vector
        Instead you need to use methods
    */

    // To see the Bytes in a string
    for bit in s4.bytes() {
        println!("{}", bit);
    }

    // To see the scalar values in a string
    for c in s4.chars() {
        println!("{}", c);
    }

    /*
        The last option is to use Graphene Clusters
        But the ability to iterate on graphene clusters is not native for
        Rust, so you need to add the unicode-segmentation crate
    */
    for g in s4.graphemes(true) {
        println!("{}", g);
    }
}
