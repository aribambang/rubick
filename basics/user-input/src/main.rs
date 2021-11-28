use std::io;

fn main() {
    // declare variable input.
    let mut input = String::new();

    println!("Say something: ");

    // assign value to variable input from terminal.
    match io::stdin().read_line(&mut input){
        Ok(_) => {
            println!("You said {}", input);
        },
        Err (e) => {
            println!("Something went wrong {}", e);
        }
    }
}
