1. Difference Between fp32 and fp64 Data Types
In Rust, floating-point numbers can be represented by two types

f32: 32-bit floating-point number 
f64: 64-bit floating-point number
            Key Differences
Bit Size:
f32 uses 32 bits to represent a number.
f64 uses 64 bits, which allows for more precision and a larger range.

Precision:
f32 gives you approximately 7 decimal digits of precision.
f64 gives you approximately 15-16 decimal digits of precision

f32 = 1.1234567; // 7 decimal places, limited precision
f64 = 1.1234567891234567; // 15-16 decimal places, high precision


Range:
f32 can represent numbers between 1.4 x 10^-45 to 3.4 x 10^38.
f64 can represent numbers between 5.0 x 10^-324 to 1.8 x 10^308

Memory Usage:
f32 uses less memory (4 bytes).
f64 uses double the memory (8 bytes).

When to Use:
we can use f32 when performance and memory efficiency are important and precision isn’t critical (e.g., graphics, gaming, some ML models).
we can use  f64 when higher precision or a larger range is needed (e.g., scientific computations, financial calculations).


2. Role of Pointers in Memory Management in Rust
In Rust, pointers are crucial for memory management, but unlike languages like C or C++, Rust provides safe memory management through its ownership model, ensuring that most pointer issues (like dangling pointers, null pointers, etc.) are handled at compile-time.

                        Key Points About Pointers in Rust


Memory Address:
In Rust, pointers store the memory address of a variable. These are like references to variables stored elsewhere in memory, allowing you to directly manipulate the data.

let x = 10;
let y = &x; // `y` is a pointer to `x`
println!("The value of x is: {}", x); // Accessing value of `x` directly

Dynamic Memory Allocation:
Rust automatically manages dynamic memory through its ownership model, but  can be  manually allocate  using Box<T>, Rc<T> (reference-counted), and Arc<T> (atomic reference-counted) for heap-allocated data.

Efficient Data Access:
By using references (essentially pointers) in Rust, data can be passed around efficiently without being copied.
let s = String::from("Hello");
let t = &s; // Borrowing a reference to `s`
println!("{}", t); // Access data without copying

Pass-by-Reference:
Rust uses references (&T) and mutable references (&mut T) to allow functions to modify variables without making copies.

Pointer Arithmetic:
Rust does not allow pointer arithmetic in safe code. This prevents mistakes like buffer overflows and out-of-bounds access. 


Memory Management:
Rust uses a borrow checker and ownership system to prevent memory leaks. When a variable goes out of scope, Rust automatically deallocates memory (destruction) without needing explicit free() or delete() commands.


in summary Pointers in Rust are used to reference data in memory efficiently without copying.
Rust’s ownership and borrowing system ensures memory safety, and dynamic memory allocation is done using Box, Rc, Arc, and other structures.
Rust doesn’t allow pointer arithmetic in safe code to prevent common errors (e.g., buffer overflows), but you can use unsafe blocks if necessary, although they should be used with caution.



difference between &str and string 

&str: This is a string slice, which is an immutable reference to a portion of a string. It does not own the data it references; instead, it borrows the data from another source, such as a String or a string literal. &str is lightweight and useful when you only need to view a part of a string without owning it. It is commonly used for function parameters when you do not need to modify the string.

String: This is a growable, heap-allocated, UTF-8 encoded string. It owns its data and can be modified and resized at runtime. You can create a new String by calling String::new() or by converting from a string slice using to_string(). String is mutable and can be used in scenarios where you need to build or modify strings dyn








A CPU, or Central Processing Unit, is the primary component in a computer that executes instructions and processes data from computer programs. It acts as the “brain” of the computer, handling tasks such as arithmetic, logic, and input/output operations. The CPU coordinates other components in the system and is essential for the computer’s overall functionality.

RAM (Random Access Memory)

Temporary, fast memory that stores:

Currently running programs,Active data being processed,Operating system components


Characteristics:

Volatile (loses data when power is off)
Much faster than storage drives
Directly accessible by CPU
Measured in GB (8GB, 16GB, etc.)



ROM (Read-Only Memory)

Permanent memory that contains:

BIOS/UEFI (Basic Input/Output System)
Firmware
Boot instructions


Characteristics:

Non-volatile (keeps data when power is off)
Can't be easily modified
Slower than RAM
Usually small capacity



How They Work Together:

ROM stores startup instructions
When computer starts, CPU reads ROM instructions
Operating system loads into RAM
CPU processes instructions, using RAM as working memory
Data flows between components via system bus


A stack is a part of memory that has a specific data structure.

When there is a new piece of data that should be stored, the stack will always put it “on top”. (push to the stack)
When a piece of data should be removed from the stack, it is also taken “from the top”. (pop off the stack)
using the   last in, first out (LIFO) data structure.
All entries that are pushed to the stack are required to have a known, fixed size. The size should be known at compile time, and can not change during runtime. We can not put things of variable size in the stack (like a string that might grow in size).


                heap 

Heap memory in Rust is a part of the memory system used for dynamic memory allocation. Unlike stack memory, which is managed automatically by the compiler and has a fixed layout, heap memory is managed manually and is not bound to function scopes. This means that variables allocated on the heap can live longer than the function that created them, allowing for more flexible memory managem


A pointer is a general concept for a variable that contains an address in memory. This address refers to, or “points at,” some other data.


anything not implemented by a vetor dont have  capacity 




