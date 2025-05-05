pub fn main() {
    // let s: String = String::from("Hello");
    // let mut rev1 = String::new();
    // for char in s.chars().rev() {
    //     rev1.push(char);
    // }
    // println!("{}", rev1);    

    // Iterators: Try .filter() to get only even numbers from a vector.
    let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let even: Vec<i32> = vec.into_iter().filter(|x| x % 2 == 0).collect();

    // let even2: Vec<i32> = vec.into_iter().filter(|x| x % 2 == 0).collect();

    // println!("Even numbers: {:?}", even2);

    // let v = vec![1, 2, 3, 4];

    // let y = v.into_iter();
    // println!("{:?}", v);
}