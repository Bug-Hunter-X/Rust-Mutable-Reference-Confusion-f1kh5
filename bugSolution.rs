fn main() {
    let mut x = 5;
    {
        let y = &mut x; // y is a mutable reference to x
        *y = 6; 
    }
    {
        let z = &mut x; // z is a mutable reference to x, but this is in a different scope
        *z = 7; 
    }
    println!("x = {}", x); // This will print 7
}

The solution uses separate scopes for the mutable references.  This ensures that there is only one mutable reference active at a time in each scope.  The borrow checker is happy, and the behavior is predictable.