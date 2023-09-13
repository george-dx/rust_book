fn main() {
    // store an i32 value on the heap
    let b = Box::new(5);
    println!("b = {b}");
}
