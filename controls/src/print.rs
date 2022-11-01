pub fn print() {
    println!("Hello, world! - From Module");
}

pub fn print_w_args(name: &str) {
    println!("Hello, {}!", name);
}

pub fn print_tuple_debug() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
    println!("The value of x is: {}", x);

    println!("The value of tuple is: {:?}", (x, y, z));
    println!("The value of tuple is: {:#?}", (x, y, z));

    println!("The value of tuple is: {:#?}", (x, y, (x,y), z));
}

pub fn print_arrays() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("The value of a is: {:?}", a);
}
