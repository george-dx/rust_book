enum SpreadshetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // Vectors
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

    let row = vec![
        SpreadshetCell::Int(3),
        SpreadshetCell::Text(String::from("blue")),
        SpreadshetCell::Float(10.12),
    ];

    // Strings
    let mut s = String::new();
    let data = "initital content";
    let ss = data.to_string();
    let sss = "initial cont".to_string();
    let ssss = String::from("initial contents");

    let hello = String::from("عليكم السلام");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("םֹלוָׁש;");
    let hello = String::from("");
    let hello = String::from("こんにちは");
    let hello = String::from("");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    let mut s = String::from("lo");
    s.push('l');
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from(" World!");
    
    // fn add(self, s: &str) -> String { -> this is how it is
    // implemented. The reason we’re able to use &s2 in the 
    // call to add is that the compiler can coerce the &String 
    // argument into a &str. 
    let s3 = s1 + &s2;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("tic");
    // we can use instead the format! macro for combining
    // string in more complicated ways
    let s = format!("{s1}-{s2}-{s3}");
    println!("{}", s);

    // The best way to operate on pieces of strings is to 
    // be explicit about whether you want characters or 
    // bytes.
    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }
}
