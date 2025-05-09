pub fn debugging() {
    let x = 5;

    dbg!(x);
    println!("The value of x is {x}");

    //    debug_only_function();
}

#[cfg(debug_assertions)]
fn debug_only_function() {
    println!("This only runs in debug builds");
}
