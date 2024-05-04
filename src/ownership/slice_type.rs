/*
    Slices allow you to reference a contiguos sequence
    of elements within a collection

    Slices do not take ownership of the data
*/

fn main(){
    let mut s: String = String::from("hello world");
    /*
        When using slices you can point to the start and end of what
        you want to slice as done in this example.

        You can also ommit the first value if you are slicing
        from the start of the collection: &s[..5] == &s[0..5]

        Same thing can be done if the slice is from index N to the
        end of the collection: &s[6..] == &s[6..11]
    */
    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];

    let fw_slice = first_word(&s);
    let s2 = "Some other word";

    let fw_slice = first_word(&s2);
}

fn first_word(s: &str) -> &str {
    let bytes: &[u8] = s.as_bytes();

    for(i, &item) in bytes.iter().enumarate(){
        if item == b' '{
            return &s[..i];
        }
    }

    &s[..]
}
