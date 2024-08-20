use std::fs;
use std::io::Error;
fn main() {
    // let text = fs::read_to_string("logs.txt");

    // println!("{:#?}", text);

    match divide(5.0, 0.0) {
        Ok(result_of_division) => {
            println!("{}", result_of_division);
        }
        Err(what_wnt_wrong) => {
            println!("{}", what_wnt_wrong);
        }
    }

    match validate_email(String::from("asdfasdf.com")) {
        Ok(..) => println!("email is valid"),
        Err(reason_this_failed_validation) => {
            print!("{}", reason_this_failed_validation)
        }
    }
}

fn validate_email(email: String) -> Result<(), Error> {
    if email.contains("@") {
        // Success!
        Ok(())
    } else {
        Err(Error::other("emails must have an @"))
    }
}

fn divide(a: f64, b: f64) -> Result<f64, Error> {
    if b == 0.0 {
        Err(Error::other("Can't divide by 0"))
    } else {
        Ok(a / b)
    }
}
