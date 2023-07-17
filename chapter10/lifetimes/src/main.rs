fn longest<'a>(x: &'a str, y:&'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let x = 5;
    let r = &x;

    println!("r: {r}");

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    let string1 = String::from("long string is long");
    // let result;

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str()); // without let
        println!("The longest string is {result}");
    }
    // println!("The longest string is {result}"); // won't compile - borrowed
    // value does not live long enough
}
