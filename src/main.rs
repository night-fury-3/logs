use std::fs;

fn main() {
    match fs::read_to_string("losgs.txt") {
        Ok(text_that_was_read) => {
            println!("{}", text_that_was_read);
        }
        Err(why_this_failed) => {
            println!("Failed to read file: {}", why_this_failed)
        }
    }
}
