# Pointers in Rust Memory Management

## Overview
Rust's pointer system is fundamental to its memory safety guarantees. Unlike C/C++, Rust provides several pointer types with different ownership and borrowing semantics.

## Types of Pointers

### References (&T and &mut T)
- Most basic form of Rust pointers
- Non-nullable
- Always valid
- Follow borrowing rules
- Zero runtime cost
- Lifetime checked at compile time

Example:
```rust
fn process(data: &u32) {
    println!("Value: {}", *data);
}
```

### Box<T>
- Heap allocation
- Single owner
- Fixed size known at compile time
- Automatically deallocated when Box goes out of scope

Example:
```rust
let boxed = Box::new(5);
```

### Raw Pointers (*const T and *mut T)
- Similar to C pointers
- No safety guarantees
- Used in unsafe code
- No automatic cleanup
- Can be null

Example:
```rust
let mut value = 10;
let raw = &mut value as *mut i32;
```

## Memory Management Features

### Ownership Rules
1. Each value has exactly one owner
2. When owner goes out of scope, value is dropped
3. Ownership can be transferred (moved)

### Borrowing Rules
1. One mutable reference OR any number of immutable references
2. References must not outlive their referent
3. No null references possible

### Smart Pointers
- Rc<T>: Reference counted, multiple owners
- Arc<T>: Atomic reference counting for thread-safety
- Weak<T>: Weak references that don't prevent cleanup

## Common Use Cases

### Stack vs Heap Allocation
```rust
// Stack allocation
let x = 5;
let y = &x;  // Reference to stack data

// Heap allocation
let boxed = Box::new(5);
let reference = &*boxed;  // Reference to heap data
```

### Dynamic Dispatch
```rust
trait Animal {
    fn make_sound(&self);
}

// Box<dyn Animal> is a pointer to any type implementing Animal
fn process_animal(animal: Box<dyn Animal>) {
    animal.make_sound();
}
```

### Recursive Data Structures
```rust
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>
}
```

## Best Practices

1. Prefer references over raw pointers
2. Use Box<T> for heap allocation
3. Consider Rc<T>/Arc<T> for shared ownership
4. Document unsafe pointer usage
5. Implement Drop trait for custom cleanup
6. Use smart pointers appropriately
7. Understand lifetimes

## Safety Considerations

1. Memory Leaks Prevention
- Rust's ownership system prevents most memory leaks
- Resource cleanup is deterministic
- Drop trait ensures proper cleanup

2. Thread Safety
- Send and Sync traits
- Arc<T> for thread-safe reference counting
- Mutex and RwLock for synchronization

3. Null Safety
- Option<T> instead of null
- Never dereferencing null pointers
- Compile-time checks

## Common Patterns

### Interior Mutability
```rust
use std::cell::RefCell;

let data = RefCell::new(5);
*data.borrow_mut() += 1;
```

### Shared Ownership
```rust
use std::rc::Rc;

let shared = Rc::new(String::from("shared data"));
let clone = Rc::clone(&shared);
```

### Safe Raw Pointer Usage
```rust
unsafe fn dangerous_operation(ptr: *mut i32) {
    if !ptr.is_null() {
        *ptr += 1;
    }
}
```