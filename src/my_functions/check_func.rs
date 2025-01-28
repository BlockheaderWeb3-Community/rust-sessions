fn main() {
    let check: bool = check_func(5, 10);
    println(check);
}
fn check_func(num1: u8, num2: u8) -> bool {
    let sum_of_two_nums = sum(num1, num2);
    if sum_of_two_nums % 2 == 0 {
        return true;
    } else {
        return false;
    }
}