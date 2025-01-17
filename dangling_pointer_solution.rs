fn main() {
    let mut v = vec![1, 2, 3];
    let ptr = v.as_mut_ptr();
    unsafe {
        *ptr = 42; // This is fine
    }
    println!("{:?}", v); // v is now [42, 2, 3]

    // Safe alternative: Use a reference
    {   //Scope that limits reference lifetime
        let v2 = vec![1,2,3];
        let value = &mut v2[0];
        *value = 100; // this is safe
        println!("{:?}", v2); // v2 is now [100, 2, 3]
    } //v2 is dropped here
}