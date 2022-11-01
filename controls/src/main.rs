mod print;

fn main() {
    // println!("Hello, world!");
    print::print();

    print::print_w_args("Rust User");

    print::print_tuple_debug();

    print::print_arrays();

    let a = [12, 28, 36, 43, 51];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }

    println!("Done! The value of index is: {}", index);


    for item in a.iter() {
        println!("the value is: {}", item);
    }

    for item in a.iter().rev() {
        println!("the value is: {}", item);
    }

    index = 0;

    let last = loop {
        index = index + 1;

        if index == 5 {
            break a[index - 1];
        }
    };

    println!("The value of last is: {}", last);
    println!("The value of index is: {}", index);

    for reverse_item in (1..4).rev() {
        println!("the value is: {}", a[reverse_item]);
    }
}
