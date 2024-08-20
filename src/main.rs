use std::fs;
use std::io::Error;
fn main() {
    let text = fs::read_to_string("logs.txt");

    println!("{:#?}", text);
}

fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("Can't divide by 0"))
    } else {
        Ok(a / b)
    }
}
