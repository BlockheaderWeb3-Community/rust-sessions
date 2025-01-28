pub fn floating_num() {
    let floating_sum: f32 = floating_sum(3.6, 8.8);
    let floating_substrt: f32 = floating_substrt(24.6, 8.8);

    println!("floating_sum result is {}", floating_sum);
    println!("floating_sum result is {}", floating_substrt);
}

pub fn floating_sum(x: f32, y: f32) -> f32 {
    x + y
}

pub fn floating_substrt(x: f32, y: f32) -> f32 {
    x - y
}