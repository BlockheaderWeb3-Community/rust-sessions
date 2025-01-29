## System Architecture

System architecture is the model that defines the structure of a computer system. This model captures the hardware, software, etc. of the systems and also how each component of the system behaves at a certain time.

It provides a comprehensive blueprint outlining how different system components interact and work together to achieve specific objectives.
## System Memory
Just like a human brain's memory, our memory makes us who we are, It helps us to remember our past, retain skills and plans for the future. A computer memory also does the same 

Everything in a computer's memory takes the form of a basic unit called bits or binary digits which each memory is stored in a binary cell that can switch between two states for two possible values 0 and 1, files and programs on a system contain millions of this bits stored in the system memory and all is processed in the center processing unit (CPU) that act as the computer brain


As the number of bits that needs to be processed grows computer designers face constant strugle between size, cost and speed
computer has a short time memory for immediate tasks and long time momory for a parmanent storage

## Types Of Memory
* Primary Memory
* Secondary Memory
* Cache Memory

### Primary Memory

It is also known as the main memory of the computer system. It is used to store data and programs or instructions during computer operations, Primary memory is of two type RAM and Rom

* RAM (Random Access Memory): It is a volatile memory. Volatile memory stores information based on the power supply. If the power supply fails/ interrupted/stopped, all the data and information on this memory will be lost. RAM is used for booting up or start the computer. It temporarily stores programs/data which has to be executed by the processor. RAM is of two types static and dynamic RAM

* ROM (Read Only Memory): It is a non-volatile memory. Non-volatile memory stores information even when there is a power supply failed/ interrupted/stopped. ROM is used to store information that is used to operate the system. As its name refers to read-only memory, we can only read the programs and data that is stored on it. It contains some electronic fuses that can be programmed for a piece of specific information. The information stored in the ROM in binary format. It is also known as permanent memory

## Cache Memory 

Cache memory is a high-speed semiconductor memory that can help the CPU run faster. Between the CPU and the main memory, it serves as a buffer. It is used to store the data and programs that the CPU uses the most frequently

## Secondary Memory

It is also known as auxiliary memory and backup memory. It is a non-volatile memory and used to store a large amount of data or information. The data or information stored in secondary memory is permanent, and it is slower than primary memory. A CPU cannot access secondary memory directly. The data/information from the auxiliary memory is first transferred to the main memory, and then the CPU can access it


## Memory In Rust 
Rust make use of two types of storage to manage data at run time, The stack and The heap 

*   Stack: stack is where rust store data that it size is known at run time, The stack stores values in the order it gets them and removes the values in the opposite order
This is referred to as last in, first out. All data stored on the stack must have a known, 
fixed size , The Data stored on the heap return an adddress which is a fixed size and it is stored on the stark it serve as a pointer to the data on the heap


*   Heap: Heap is where Data with an unknown size at compile time or a size that might change is stored on
The heap is less organized: when you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location.