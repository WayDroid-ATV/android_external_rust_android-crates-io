use std::env;

fn main() -> Result<(), ()> {
    for argument in env::args() {
        println!("{argument}");
    }
    // Err(())
    Ok(())
}
