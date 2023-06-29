fn main() {
    let _v1: Vec<i32> = Vec::new();
    let _v2 = vec![1, 2, 3];

    let mut v3 = Vec::new();

    v3.push(5);
    v3.push(6);
    v3.push(7);

    println!("Vector v3 is {:?}", v3);

    let v4 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v4[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v4.get(22);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
        // for mut i ... -> this won't work because
        // i = &(i + 1);
        // the temporary value is freed here while still in use
        // println!("{:?}", v[0]);
        // here - borrow later used here
        // println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
    println!("Vector v is {:?}", v);
}
