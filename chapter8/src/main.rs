use std::collections::HashMap;

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

    let _row = vec![
        SpreadshetCell::Int(3),
        SpreadshetCell::Text(String::from("blue")),
        SpreadshetCell::Float(10.12),
    ];

    // Strings
    let mut _s = String::new();
    let data = "initital content";
    let _ss = data.to_string();
    let _sss = "initial cont".to_string();
    let _ssss = String::from("initial contents");

    let _hello = String::from("عليكم السلام");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("םֹלוָׁש;");
    let _hello = String::from("");
    let _hello = String::from("こんにちは");
    let _hello = String::from("");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");

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
    let _s3 = s1 + &s2;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let _s = s1 + "-" + &s2 + "-" + &s3;

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

    // Hash maps
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    // get returns an Option<&V>, copied to get an Option<i3>
    // rather than an Option<&i32>, then unwrap_or to set
    // score to zero if scores doesn't have an entry for the
    // key
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{}", score);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();

    map.insert(field_name, field_value);

    // this is invalid since that keys and values are owned
    // by the hash map once they are inserted
    // println!("{field_value}");

    // Overwriting a value
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // Adding a key and value only if a key isn't present
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // Updating a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
