fn main() {
    println!("Hello, world!");

    // Formatting
    println!("My name is {} and I'm {} years old", "Ari", 24);

    // Expressions
    println!("a + b = {}", 2 + 10);

    // Positional arguments
    println!("{0} has a {2} and {0} has a {1}", "Ari", "rabbit", "fish");

    // Named arguments
    println!("{name} {surname}", surname="Bambang", name="Ari");

    // Printing traits
    println!("binary: {:b}, Hex: {:x}, Octal: {:o}", 23, 23, 23);

    // Debug
    println!("Array: {:?}", [1, 2, 3]);
}
