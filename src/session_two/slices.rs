pub fn main() {
    let a = [1, 2, 3, 4, 5, 6];

    let slice = &a[1..4];

    println!("Slice from a: {:?}", slice);
}