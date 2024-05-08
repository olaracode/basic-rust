/*
    Generic Types allows us to use the same logic,
    be it a function,struct, enum, etc with different data types
    making it receive a 'dynamic type'
*/

/*
    This struct only accepts i32 values
    so if we were to need to use a floating number
    we wouldn't be able to do so, so we would have to create
    another struct
*/
struct Point {
    x: i32,
    y: i32,
}

/*
    Instead we can use generic types inside structs
*/
struct GPoint<T> {
    x: T,
    y: T,
}

/*
    We can also use two or more generic types
    so that our struct can handle different types for each
    field
*/
struct GUPoint<T, U> {
    x: T,
    y: U,
}

fn main() {
    let p1 = Point { x: 5, y: 6 };

    let gp1 = GPoint { x: 5, y: 6 };
    let gp2 = GPoint { x: 1.5, y: 2.6 };

    let gu1 = GUPoint { x: 5, y: 1.5 };
    let gu2 = GUPoint { x: 'c', y: 3 };
}
