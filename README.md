# Rust Sessions
Intro to Primitives

[Detailed Introductory Resources](https://hackmd.io/93HrBDYsTyql7IGuxw4nzw?view)

# Floating Points article
## Diffrence between f64 and f32
In Rust, f32 and f64 are floating-point number types that differ 
in precision and memory usage:

floating point are always signer - to +
the default type of a floating point is f64
even when assigned as an f32 or an f64 the number still has to have a point to indicate that it is a floating number
dont compare two floating point nums because they have limited precision

### f32 (Single Precision)
- 32 bits wide
- Less precise
- Uses less memory
- Suitable for graphics and when memory is constrained

### f64 (Double Precision)
- 64 bits wide
- More precise
- Modern default choice
- As fast as f32 on most current hardware

### Key Considerations
- Use f64 by default for better precision
- f32 to f64 conversion is lossless 
- f64 to f32 conversion may lose precision
- No implicit conversions between types
- Use `as` keyword or `From` trait for type conversion

For specialized needs:
- `num-rational` crate: infinite-precision
- `rust_decimal` crate: fixed-precision decimals