fn while_example(){
    let mut x = 200;
    while x >= 10 {
        x = x /2;
    }
    println!("Final x: {x}");
}

fn for_example(){
    for x in 1..5{
        println("x: {x}");
    }

    let list = [1,2,3,4,5];
    for elem in list{
        println!("Elem: {elem}");
    }
}

// 'loop' loops allow you to iterate until
// a break statement is met
// You can also use the continue statement to carry
// on to the next cycle of the loop
fn loop_example(){
    let mut i = 0;
    loop {
        i += 1;
        println!("{i}");
        if i > 100 {
            break;
        }
    }
}

fn main(){
    while_example();
    for_example();
    loop_example();
}
