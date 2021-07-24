macro_rules! cool_println {
    // This macro takes an expression of type `expr` and prints it
    // The `expr` designator is used for expressions.
    ($line_to_print:expr) => {
        println!("**{}**", $line_to_print);
    }
}

fn main() {
    println!("Hello, world!");
    cool_println!("Hello, world!");
}
