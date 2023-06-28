use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
    return Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    return Ok(())
}


fn main() {
    println!("Hello, world!");
}
