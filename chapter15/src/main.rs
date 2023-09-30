use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

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

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    // store an i32 value on the heap
    let b = Box::new(5);
    println!("b = {b}");

    // let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

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

    // let list1 = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // println!("Count after creating list1 = {}", Rc::strong_count(&list1));
    // let list2 = Cons(3, Rc::clone(&list1));
    // println!("Count after creating list2 = {}", Rc::strong_count(&list1));
    // {
    //     let list3 = Cons(4, Rc::clone(&list1));
    //     println!("Count after creating list3 = {}", Rc::strong_count(&list1));
    // }
    // println!(
    //     "Count after list3 goes out of scope = {}",
    //     Rc::strong_count(&list1)
    // );

    let value = Rc::new(RefCell::new(5));

    let aa = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let bb = Cons(Rc::new(RefCell::new(3)), Rc::clone(&aa));
    let cc = Cons(Rc::new(RefCell::new(4)), Rc::clone(&aa));

    *value.borrow_mut() += 10;

    println!("aa after = {:?}", aa);
    println!("bb after = {:?}", bb);
    println!("cc after = {:?}", cc);
}
