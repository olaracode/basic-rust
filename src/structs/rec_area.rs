
/*
    Using derive allows to pass the Debug trait to our struct
*/
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

/*
    Implementation blocks
    will house the functions | methods
    associated to our struct
*/
impl Rectangle {
    fn area(&self) -> u32 {
        self.width *  self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

/*
    Structs can also hold associative functions
    which are not tied to the instance of the Struct
    but to the Struct it self.

    Associative functions dont receive the &self value
    Associative functions can be declared in the same implementation block
    as the Struct methods.

    In this instance we separated them to make it clear that Rust allows
    the use of various impl blocks for the same struct
*/
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main(){
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Rectangle {:#?}": rect);

    println!(
        "The area of the rectangle is {} square pixels",
        rect.area();
    );

    let comp_rect = Rectangle {
        width: 20,
        height: 30
    };

    let comp_rect2 = Rectangle {
        width: 50,
        height: 60
    };

    println!(
        "Can the first rect hold the second? {}",
        rect.can_hold(&comp_rect)
    );

    println!(
        "Can the first rect hold the third? {}",
        rect.can_hold(&comp_rect2)
    );

    let rect_from_square = Rectangle::square(10);
    println!(
        "Rectangle created from square value {:#?}",
        rect_from_square
    )
}

/*
    As soon as the implementation block with the area function is created
    this function is no longer required
*/
fn calc_area(dimensions: &Rectangle) -> u32 {
    dimensions.width * dimensions.height
}
