## Memory
Memory is where the aspect of a machine that data is stored and processed, Rust store data using stack andd heap,  stack and the heap are parts of memory available to your code to use at runtime,but they are structured in different ways

## The Stack and Heap
### Stack

- The stack stores values in the order it gets them and removes the values in the opposite order, This is referred to as last in, first out, The stack is where data with known size at compile time is stored

### Heap

- The heap is where data with an unknown size at compile time or a size that might change is stored, unlike the stack the heap is less organized, A certain amount of space is requsted when data is stored on the heap. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer,

## Pointer

A pointer is an address stored on the stack, the address is what is been used by rust to point to a data on the heap where the actual value (data) is stored, the pointer is a value that is known at compile time beacause the pointer to the heap has a fixed size at compile time, but when we want to get the data attach to the pointer, the pointer is first located on the stack when such pointer is gotten the pointer is used to get the value of the pointer that is stored on the heep