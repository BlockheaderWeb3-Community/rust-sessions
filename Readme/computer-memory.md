

# Computer System Architecture

## CPU (Central Processing Unit)

### Core Components

#### 1. Control Unit (CU)
- Coordinates all CPU operations
- Fetches and decodes instructions
- Controls data flow between components

#### 2. Arithmetic Logic Unit (ALU)
- Handles mathematical calculations
- Performs logical operations
- Processes data according to instructions

#### 3. Registers
- **Program Counter**: Tracks next instruction
- **Instruction Register**: Holds current instruction
- **Accumulator**: Stores calculation results

---

## RAM (Random Access Memory)

### Key Features

#### 1. Volatile Memory
- Data is temporary
- Erased when power is off

#### 2. Performance
- Fast read/write speeds
- Direct CPU access
- Enables multitasking

#### 3. Usage
- Stores active programs
- Holds current working data
- Manages running applications

---

## ROM (Read Only Memory)

### Characteristics

#### 1. Non-Volatile Memory
- Permanent storage
- Retains data without power

#### 2. Contents
- **BIOS/UEFI**: Boot instructions
- **Firmware**: Basic hardware instructions
- **System Instructions**: Core operations

#### 3. Types
- **PROM**: Programmable ROM
- **EPROM**: Erasable PROM
- **EEPROM**: Electrically Erasable PROM

---

## System Integration

### Data Flow
```
CPU ⟷ Cache ⟷ RAM ⟷ Storage ⟷ ROM
```

### Memory Hierarchy
1. **CPU Registers** (Fastest)
2. **Cache Memory**
3. **RAM**
4. **Storage (HDD/SSD)**
5. **ROM** (Slowest)

### Bus Architecture
- **Data Bus**: Transfers actual data
- **Address Bus**: Specifies memory locations
- **Control Bus**: Carries control signals

---

## Performance Considerations

### RAM Impact
- More RAM = Better multitasking
- Faster RAM = Quicker data access
- Insufficient RAM = System slowdown

### CPU Factors
- **Clock speed** affects processing speed
- **Multiple cores** enable parallel processing
- **Cache size** impacts data access speed

---

# Rust Memory Management System

## Memory Layout Overview

### 1. Stack Memory

#### Characteristics:
- Fixed-size, LIFO (Last In, First Out) structure
- Fast access and automatic memory management
- Size known at compile time
- Memory automatically freed when variables go out of scope

```rust
fn example() {
    let x = 42;             // Integer stored on stack
    let y = true;           // Boolean stored on stack
    let array = [1, 2, 3];  // Fixed-size array on stack
}  // All variables automatically deallocated here
```

### 2. Heap Memory

#### Characteristics:
- Dynamic size allocation
- Slower access than stack
- Manual management (in most languages, but Rust handles this through ownership)
- Memory persists until explicitly deallocated

```rust
fn heap_example() {
    let string = String::from("Hello"); // Allocated on heap
    let vector = Vec::new();            // Dynamic array on heap
    let box_value = Box::new(42);       // Boxing moves value to heap
}  // Memory automatically freed due to Rust's ownership system
```

### 3. Data Section

#### Static Memory:
- Stores global variables
- Constants and static variables
- String literals
- Available throughout program lifetime

```rust
static GLOBAL: i32 = 42;            // Static variable
const MAX_POINTS: u32 = 100_000;    // Constant
static mut COUNTER: u32 = 0;        // Mutable static (unsafe)
```

---

## Rust's Unique Memory Features

### 1. Ownership Rules
- Each value has exactly one owner
- When owner goes out of scope, value is dropped
- Ownership can be transferred (moved)

```rust
fn ownership_example() {
    let s1 = String::from("hello");  // s1 owns the string
    let s2 = s1;                     // ownership moved to s2
    // println!("{}", s1);           // Would cause compile error
}
```

### 2. Borrowing
- References allow temporary access without ownership transfer
- Two types of borrows:
  - **Shared references (`&T`)**
  - **Mutable references (`&mut T`)**

```rust
fn borrow_example() {
    let mut s = String::from("hello");
    let r1 = &s;        // Shared reference
    let r2 = &s;        // Multiple shared references allowed
    let r3 = &mut s;    // Only one mutable reference allowed
}
```

### 3. Lifetimes
- Ensure references are valid
- Prevent dangling references
- Usually inferred by compiler

```rust
fn lifetime_example<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

---

## Memory Management Features

### 1. Smart Pointers
- **`Box<T>`**: Single-owner heap allocation
- **`Rc<T>`**: Reference-counted shared ownership
- **`Arc<T>`**: Atomic reference counting for threads

```rust
use std::rc::Rc;

let shared = Rc::new(String::from("shared data"));
let clone = Rc::clone(&shared);  // Reference count increased
```

### 2. Drop Trait
- Automatic cleanup when values go out of scope
- Custom cleanup logic possible

```rust
struct CustomType;

impl Drop for CustomType {
    fn drop(&mut self) {
        println!("Cleaning up!");
    }
}
```

### 3. Memory Safety Guarantees
- No null pointers
- No dangling references
- No data races
- Memory leaks prevented (in most cases)
- Thread safety through type system

```rust
fn safety_example() {
    let v = vec![1, 2, 3];
    let first = &v[0];  // Reference to first element
    // v.clear();       // Would cause compile error
    println!("{}", first);
}
```

This system ensures memory safety without garbage collection, making Rust both safe and performant. The compiler enforces these rules at compile time, preventing most memory-related bugs before the program runs.
