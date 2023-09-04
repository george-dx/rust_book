use art_dx::{mix, PrimaryColor, SpecialColor};

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);

    let white = SpecialColor::White;
    println!("{:?} is a special color!", white);
}
