
pub fn name(x: char, y: char) -> String {
    let mut name = String::new();
    name.push(x);
    name.push(y);
    name
}

 pub fn myname(x: &str, y: &str) -> String {
    let mut myname = String::new();
    myname.push_str(x);
    myname.push_str(y);
    myname
}

pub fn intro_to_char() {
    let name_result: String = name('D', 'o');
    println!("The char result is: {}", name_result);

    let myname_result: String = myname("Darey", "Olowo");
    println!("my name are : {}", myname_result);
}