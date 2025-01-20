
    # Rust also has two primitive types for floating-point numbers, which are numbers with decimal points. 
        Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively.
        The default type is f64 because on modern CPUs, it’s roughly the same speed as f32 but is capable of more precision.
    # Differences Between 'f32' and 'f64' in Rust
        The 64-bit floating-point type is very similar to f32, but has increased precision by using twicw as many bits
  ## Precision
    - **`f32`**: 32 bits (4 bytes), approximately **6–7 decimal digits** of precision.
    - **`f64`**: 64 bits (8 bytes), approximately **15–16 decimal digits** of precision.

    ## Memory Usage
    - **`f32`**: 4 bytes.
    - **`f64`**: 8 bytes.

    ## Performance
    - **`f32`**: Faster for some operations and uses less memory.
    - **`f64`**: More precise but slightly slower.

    ## Default Type
    - Rust defaults to `f64` for floating-point literals unless explicitly annotated.

