fn evaluate_num_size(n: i32){
    println!("Evaluation of {n}");
    if n == 0 {
        println!("Zero!");
    } else if n < 100 {
        println!("Biggish")
    } else {
        println!("Huge")
    }
}

fn evaluate_one_liner(n: i32){
    let size = if n < 20 { "Small" } else { "Large" } ;
    println!("Number Size: {size}");
}

fn main(){
    evaluate_num_size(10);
    evaluate_num_size(100);
    evaluate_num_size(1000);
    evaluate_one_liner(20);
    evaluate_one_liner(10)
}
