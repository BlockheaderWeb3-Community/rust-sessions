fn main() {
    intro_to_u();
    intro_to_f();
    intro_to_char();
    convert_high();
    convert_high_to_low();
    perform_operations();
}

// function to encapsulate all integers and to check for even numbers
fn intro_to_u() {
    let sum_result: u8 = sum(5, 20);
    println!("the sum result is: {}", sum_result);
    println!(
        "Is the sum result even? {}",
        if is_even(sum_result) { "Yes" } else { "No" }
    );

    let subtract_result: u8 = subtract(20, 15);
    println!("the subtract result is: {}", subtract_result);
    println!(
        "Is the subtract result even? {}",
        if is_even(subtract_result) { "Yes" } else { "No" }
    );

    let multiply_result: u8 = multiply(5, 12);
    println!("the multiply result is {}", multiply_result);
    println!(
        "Is the multiply result even? {}",
        if is_even(multiply_result) { "Yes" } else { "No" }
    );


    let divide_result: u8 = divide(20, 4);
    println!("the divide result is {}", divide_result);
    println!(
        "Is the divide result odd? {}",
        if is_odd(divide_result) { "Yes" } else { "No" }
    );
    



}

// functions for Floating points

fn intro_to_f() {
    let add_result: f64 = add(10.3.into(), 10.1.into());
    println!("the add result is: {}", add_result);
  

    let minus_result: f64 = minus(25.5.into(), 5.5.into());
    println!("the subract2 result is: {}", minus_result);

    let rise_result: f64 = rise(5.2.into(), 3.7.into());
    println!("the multiply2 result is {}", rise_result);

    let split_result: f64 = split(30.9.into(), 2.3.into());
    println!("the divide result is {}", split_result);
}

// function char

fn intro_to_char() {
    let name_result: String = name('D', 'o');
    println!("The char result is: {}", name_result);

    let myname_result: String = myname("Darey", "Olowo");
    println!("my name are : {}", myname_result);
}

// function to encapsulate all integers

fn sum(x: u8, y: u8) -> u8 {
    x + y // implicit return
          //    return x + y; // explicit return
}

// subtract

fn subtract(x: u8, y: u8) -> u8 {
    return x - y;
}

// multiplication

fn multiply(x: u8, y: u8) -> u8 {
    x * y
}
// division

fn divide(x: u8, y: u8) -> u8 {
    x / y
}

// Floating-point Types

fn add(x: f64, y: f64) -> f64 {
    x + y
}

fn minus(x: f64, y: f64) -> f64 {
    x - y
}

fn rise(x: f64, y: f64) -> f64 {
    x * y
}

fn split(x: f64, y: f64) -> f64 {
    x / y
}

// char type

fn name(x: char, y: char) -> String {
    let mut name = String::new();
    name.push(x);
    name.push(y);
    name
}

fn myname(x: &str, y: &str) -> String {
    let mut myname = String::new();
    myname.push_str(x);
    myname.push_str(y);
    myname
}

//



// function to check for even numbers 

fn is_even(x: u8) -> bool {
    x % 2 == 0
}


// to check for odd nummbers

fn is_odd(x: u8) -> bool {
    x % 2 != 0
}





// how to convert u8 to u32

 fn convert_high(){
    let low: u8 = 255;
    let high: u32 = low as u32;
    println!("u8 value: {}, u32 value:{}", low, high );
 }



//Function to convert high bit integer to low bit integer u32 to u8

fn convert_high_to_low() {
    let high: u32 = 300; 
    let low: u8 = high as u8; // Convert u32 to u8 with truncation
    println!("u32 value: {}, u8 value: {}", high, low);
}



//Implement arithmetic operations on signed integers

fn perform_operations() {
    let a: i32 = 25; // A signed 32-bit integer
    let b: i32 = -10; // Another signed 32-bit integer

    // Arithmetic operations
    let sum = a + b; // Addition
    let difference = a - b; // Subtraction
    let product = a * b; // Multiplication
    let quotient = a / b; // Division
    let remainder = a % b; // Modulus (remainder)

    println!("Arithmetic Operations on signed integers:");
    println!("a = {}, b = {}", a, b);
    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);
    println!("Remainder: {}", remainder);
}



//Explain the difference between the fp32 and fp64 data types

     //The key difference between fp32 (32-bit floating-point) and fp64 (64-bit floating-point) lies in their precision and range:

       //1. Bit Size:
              // fp32 (32-bit): Uses 32 bits to represent a floating-point number.
              // fp64 (64-bit): Uses 64 bits, allowing for more precision and a larger range.

        // 2. Precision:
               // fp32: Provides approximately 7 decimal digits of precision.
              // fp64: Provides approximately 15-16 decimal digits of precision.
              // Example: Calculations with very small differences or many decimal places will be more accurate with fp64.

        // 3. Range:
               //fp32: Can represent numbers from approximately 1.4 x 10^-45 to 3.4 x 10^38.
               //fp64: Can represent numbers from approximately 5.0 x 10^-324 to 1.8 x 10^308.
               //fp64: Covers a wide range than fp32, making it suitable for scientific calculations or financial applications.

        //4. Memory Usage:
                 // 32: Uses less memory, making it faster for applications where precision isn't critical (e.g., graphics or simulations).
                 // 64: Requires double the memory of fp32, which may impact performance or storage in large datasets.

        //5. Performance:
                // 32: Faster due to smaller size, often used in real-time systems like games or machine learning models.
               // 64: Slower but more precise, used in scientific computing, financial applications, or any domain requiring high precision.

        //6. When to Use:
              // 32: Use when speed and memory efficiency are more important than precision (e.g., 3D graphics, neural networks).
              // 64: Use when precision and range are crucial (e.g., scientific simulations, high-precision calculations).

//Describe in details the role of pointers in memory management

   //Pointers play a critical role in memory management in programming, particularly in low-level languages like C, C++, and systems programming languages such as Rust (though Rust manages pointers more safely). Here’s a detailed explanation of their roles:

    //1. Memory Address:
        // Pointers store the memory address of a variable or data structure in the computer's memory. This address points to the location where the data is stored, allowing programs to access and manipulate it directly.

    //2. Dynamic Memory Allocation:
        // Pointers enable dynamic memory allocation, where programs can request memory from the heap at runtime and manage it using pointers. This allows for flexible memory usage and efficient storage of data structures like arrays, linked lists, and trees.

    //3. Efficient Data Access:
        // By using pointers, programs can directly access and modify data in memory without copying it, leading to faster and more efficient data processing. Pointers are crucial for tasks like iterating over arrays, traversing linked lists, or accessing hardware registers.

    //4. Pass-by-Reference:
        // Pointers enable pass-by-reference in function calls, where functions can modify variables in the calling scope by passing pointers to them. This allows for efficient sharing and manipulation of data across different parts of a program.

    //5. Pointer Arithmetic:
        // Pointers support arithmetic operations like addition, subtraction, and comparison, allowing for efficient memory traversal and manipulation. This is essential for tasks like iterating over arrays, implementing data structures, or working with complex data layouts.

    //6. Memory Management:
        // Pointers play a key role in memory management, allowing programs to allocate, deallocate, and reallocate memory dynamically. This helps optimize memory usage, prevent memory leaks, and manage resources efficiently in applications.

    //7. Low-Level Programming:
        // In low-level programming, pointers are essential for interacting with hardware, managing memory layouts, and implementing efficient algorithms. They provide direct access to memory locations, enabling fine-grained control over data and system resources.

    //8. Pointer Safety:
        // While pointers offer powerful capabilities, they can also introduce risks like null pointer dereferencing, memory leaks, and buffer overflows. Modern languages like Rust incorporate safety features to prevent common pointer-related errors and ensure memory safety.

    //In summary, pointers are fundamental to memory management in programming, enabling efficient data access, dynamic memory allocation, pass-by-reference semantics, and low-level system interactions. Understanding and using pointers effectively is crucial for developing performant, memory-efficient, and reliable software applications.









