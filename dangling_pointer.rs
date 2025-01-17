fn main() {
    let mut v = vec![1, 2, 3];
    let ptr = v.as_mut_ptr();
    unsafe {
        *ptr = 42; // This is fine
    }
    println!("{:?}", v); // v is now [42, 2, 3]

    // Dangling pointer
    drop(v); 
    unsafe {
        *ptr = 100; // This is UB! v is dropped and we have dangling pointer
    }
}