fn main() {
    println!("Hello, world!");
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    //cannot assign twice to immutable variable
    // x = 10;
    //cannot assign twice to immutable variable
    // y = 10;

    let a = 5;
    let b = &a;
    let c = &a;
    //cannot borrow `a` as mutable because it is also borrowed as immutable
    // let d = &mut a;
    // same as above
    // let mut d = &mut a;

    //make e mutable and borrow a mutable version
    let mut e = 5;
    let f = &mut e;
    *f += 1;

    println!("a = {}, b = {}, c = {}", a, b, c);
    println!("a = {}, b = {}, c = {}", a, *b, *c);
    println!("f = {}", f);

    let mut g = 5;

    // let h = &g;
    //cannot assign to `g` because it is borrowed
    // g = 6;

    {
        let h = &mut g;
        //this change will not be reflected in the final print statement due to scope
        *h += 1;
        println!("h = {}", h);
        // println!("g = {}", g);
    }

    //can modify g here since h is out of scope
    g += 1;

    //h is no longer in scope and cannot be used
    println!("g = {}", g);

    let mut i = 5;
    let j = i;

    i = 6;

    println!("i = {}, j = {}", i, j);
}
