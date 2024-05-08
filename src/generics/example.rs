/*
    Generic Types allows us to use the same logic,
    be it a function,struct, enum, etc with different data types
    making it receive a 'dynamic type'
*/

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    let p1 = Point { x: 5.0, y: 10.0 };
}
