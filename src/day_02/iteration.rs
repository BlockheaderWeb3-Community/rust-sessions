pub fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let iter = v.iter();

    for val in iter {
        println!("{:?}", val);
    }

    // Iterate over the value of v and add 1 to each

    let plus1: Vec<i32> = v.iter().map(|x| x + 1).collect();

    println!("{:?}", plus1);
}