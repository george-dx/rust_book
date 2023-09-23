use crate::List::{Cons, Nil};
use std::ops::Deref;

struct MyBox<T>(T);

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    // store an i32 value on the heap
    let b = Box::new(5);
    println!("b = {b}");

    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    // let y = &x;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let w = MyBox::new(x);
    assert_eq!(5, *w);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
