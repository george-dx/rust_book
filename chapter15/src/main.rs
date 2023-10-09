use crate::List::{Cons, Nil};
use std::borrow::Borrow;
use std::cell::{Ref, RefCell};
use std::ops::Deref;
use std::rc::{Rc, Weak};

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

// #[derive(Debug)]
// enum List {
//     Cons(Rc<RefCell<i32>>, Rc<List>),
//     Nil,
// }

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
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
    let _d = CustomSmartPointer {
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

    // let value = Rc::new(RefCell::new(5));

    // let aa = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    // let bb = Cons(Rc::new(RefCell::new(3)), Rc::clone(&aa));
    // let cc = Cons(Rc::new(RefCell::new(4)), Rc::clone(&aa));

    // *value.borrow_mut() += 10;

    // println!("aa after = {:?}", aa);
    // println!("bb after = {:?}", bb);
    // println!("cc after = {:?}", cc);

    let original_list = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!(
        "original_list initial rc count = {}",
        Rc::strong_count(&original_list)
    );
    println!("original_list next item = {:?}", original_list.tail());

    let copy_list = Rc::new(Cons(10, RefCell::new(Rc::clone(&original_list))));
    println!(
        "original_list rc count after copy_list creation = {}",
        Rc::strong_count(&original_list)
    );
    println!(
        "copy_list initial rc count = {}",
        Rc::strong_count(&copy_list)
    );
    println!("copy_list next item = {:?}", copy_list.tail());

    if let Some(link) = original_list.tail() {
        *link.borrow_mut() = Rc::clone(&copy_list);
    }
    println!(
        "copy_list rc count after changing original_list = {}",
        Rc::strong_count(&copy_list)
    );
    println!(
        "original_list rc count after changing original_list = {}",
        Rc::strong_count(&original_list)
    );

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack.
    // println!("original_list next item = {:?}", original_list.tail());

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf strong = {}, weak={}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("branch strong = {}, weak={}", Rc::strong_count(&branch), Rc::weak_count(&branch));

        println!("leaf strong = {}, weak={}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
    }
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong = {}, weak={}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));
}
// 18793
