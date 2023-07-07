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
}
