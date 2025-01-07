fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &mut x; // z is ALSO a mutable reference to x
    *y = 6;
    *z = 7;
    println!("x = {}", x); // This will print 7, the last assignment wins
}

This code compiles and runs without any errors, but the behavior might be unexpected if you don't understand how mutable references work in Rust.  The key here is that only ONE mutable reference to a given variable can exist at a time. However, in this code, there are two mutable references, y and z. This is only allowed if there is a guarantee that the references are disjoint.  In this case, there isn't such a guarantee, hence the seemingly arbitrary behavior (the last assignment wins).