fn main() {
    println!("Hello, world!");

    another_function(); //  You can call functions above the actual functions

    number_functions(20);
}

fn another_function() {
    println!("Another function.");
}

fn number_functions(x: i32) {
    println!("THe value of x is {}", x);
}