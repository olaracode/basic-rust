/*
    Generic Types allows us to use the same logic,
    be it a function,struct, enum, etc with different data types
    making it receive a 'dynamic type'
*/

fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let char_list = vec!['y', 'm', 'c', 'z'];
    let num_list = vec![10, 12, 13, 14];
    let largest_char = get_largest(char_list);
    let largest_num = get_largest(num_list);
    println!("{} | {}", largest_char, largest_num);
}
