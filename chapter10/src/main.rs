fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Only works with the same type for x and y
struct Point<T> {
    x: T,
    y: T,
}

struct ComplexPoint<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct MixedPoint<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> MixedPoint<X1, Y1> {
    fn mixup<X2, Y2>(
        self,
        other: MixedPoint<X2, Y2>,
    ) -> MixedPoint<X1, Y2> {
        MixedPoint {
            x: self. x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");

    let integer_point = Point {x: 5, y: 10};
    let float_point = Point {x: 1.0, y: 4.0};
    // let wont_work = Point {x: 5, y: 4.0};

    let integer_float_point = ComplexPoint {x: 5, y:4.0 };

    let p = Point {x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p1 = MixedPoint {x: 5, y: 10.4};
    let p2 = MixedPoint {x: "Hello", y: 'c'};

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
