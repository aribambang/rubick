#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main() {
    let name = "Ari Bambang";
    let mut age = 24;

    /*
        Rust is a strongly type language
        Variable type is optional if it can be inferred
        Type can be specified explicitly

        let amount = 9249893285934; -> error
    */ 

    let amount: i64 = 9249893285934;

    /*
        Names can contain letters, numbers and underscores
        Must start with a letter or underscore
        Follow snake_case naming convention
        Immutable by default

        let length = 34;
        length = 35; -> error

        Can be declared mutable

        let mut length = 34;
    */

    age = 25;

    /* 
        Shadowing is allowed

        let color = "blue";
        let color = "red";
        println!("Color is {}", color); -> Color is red

    */

    let color = "blue";
    let color = 86;
    println!("{}", color);

    /* 
        Declaring multiple variables simultaneously
        let (a, b, c) = (1, 2, 3);
    */

    let (a, b, c) = ("ari", 2, 23);
}
