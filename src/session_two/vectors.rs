pub fn main() {
    // let mut v = Vec::new();
    // v.push(1);
    // v.push(2);

    // let firstel = v[1];
    // print!("First element: {}", firstel);

    // let secondel = v.get(4);
    // println!("The second element: {:?}", secondel);

    // let v2 = vec![1, 2, 3, 4, 5];

    // println!("{:?}", v);
    // println!("{:?}", v2);

    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];

    for num in &mut v {
        *num += 20;
        println!("{}", num)
    }
}