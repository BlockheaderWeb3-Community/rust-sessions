
//mod strings;
mod float;
mod signed;
mod string;
mod unsigned;
fn main() {
    unsigned::intro_to_u();
    signed::intro_to_i();
    float::intro_to_float();
    string::strings();
}









/////////////////////////////////////////Assignment 1

// fn name_format(){
//     let full_name = string_formatting(&convert_to_string_v1("Ebube"), &convert_to_string_v2("Onuora"));
//     println!("Full Name: {}", full_name);
// }

// fn string_formatting(first_name: &str, last_name: &str) -> String {
//     let full_name = format!("{} {}", first_name, last_name);
//     return full_name;
// }

// // util fn version 1 to convert &str to String 
// fn convert_to_string_v1(x: &str) -> String {
//     x.to_string()
// }

// // util fn version 2 to convert &str to String 
// fn convert_to_string_v2(x: &str) -> String {
//     x.to_string()
// }






// ////////////////////////////////////////////////Assignment 2
// fn main (){
//     name_format();
//     convert_low_to_high();
//     convert_high_to_low();
//     perform_operations();
   
// }


// //Function to convert low integer to high integer u8 to u32
// fn convert_low_to_high(){
//     let low: u8 = 255; //A u8 value
//     let high: u32 = low as u32; //Covert u8 to u32
//     println!("u8 value: {}, u32 value:{}",low, high);

// }


// //Function to convert high bit integer to low bit integer u32 to u8
//     fn convert_high_to_low() {
//         let high: u32 = 300; // A u32 value
//         let low: u8 = high as u8; // Convert u32 to u8 with truncation
//         println!("u32 value: {}, u8 value: {}", high, low);
//     }

// //Implement arithmetic operations on signed integers

// fn perform_operations() {
//     let a: i32 = 25; // A signed 32-bit integer
//     let b: i32 = -10; // Another signed 32-bit integer

//     // Arithmetic operations
//     let sum = a + b; // Addition
//     let difference = a - b; // Subtraction
//     let product = a * b; // Multiplication
//     let quotient = a / b; // Division
//     let remainder = a % b; // Modulus (remainder)

//     println!("Arithmetic Operations on signed integers:");
//     println!("a = {}, b = {}", a, b);
//     println!("Sum: {}", sum);
//     println!("Difference: {}", difference);
//     println!("Product: {}", product);
//     println!("Quotient: {}", quotient);
//     println!("Remainder: {}", remainder);
// }

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