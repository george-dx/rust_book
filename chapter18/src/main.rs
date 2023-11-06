fn main() {
    let first_part = false;
    if first_part {
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        if let Some(color) = favorite_color {
            println!("Using your favorite, {color}, as the background");
        } else if is_tuesday {
            println!("Tuesday is green day!");
        } else if let Ok(age) = age {
            if age > 30 {
                println!("Using purple as the background color");
            } else {
                println!("Using orange as the background color");
            }
        } else {
            println!("Usingblue as the background color");
        }

        let mut stack = Vec::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        while let Some(top) = stack.pop() {
            println!("{top}");
        }

        let v = vec!['a', 'b', 'c'];

        for (index, value) in v.iter().enumerate() {
            println!("{value} is at index {index}");
        }
    }

    let x = 1;
    
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let y = Some(5);
    let z = 10;

    match y {
        Some(50) => println!("Got 50"),
        Some(z) => println!("Matched, z = {z}"),
        _ => println!("Default case, y = {:?}", y),
    }

    println!("at the end: y = {:?}, z = {z}", y);
}
