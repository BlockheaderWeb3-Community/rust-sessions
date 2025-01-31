pub fn strings() {
    intro_to_ownable_string();
    intro_to_str_slice();
    string_data_type_converter();

    let name = "yunus".to_string();
    let conversion_one = string_to_str_string_data_type_converter(&name);
    println!("str data type to string {}", conversion_one);

    let conversion_two = str_to_string_string_data_type_converter("yunus");
    println!("str data type to string {}", conversion_two);

    let format_string: String = string_formatting("yunus", " abdul");
    println!("string format {}", format_string);

    println!("-------- string conversion other method--------");
    let conversion_one = convert_to_string_v1(&name);
    println!("str data type to string {}", conversion_one);

    let conversion_two = convert_to_string_v2("yunus");
    println!("str data type to string {}", conversion_two);
}

// util fn version 1 to convert &str to String
fn convert_to_string_v1(x: &str) -> String {
    x.to_string()
}

// util fn version 2 to convert &str to String
fn convert_to_string_v2(x: &str) -> String {
    String::from(x)
}

// // handle all string-related functions
// fn string_handler() {
//     // intro_to_str_slice();
//     intro_to_ownable_string();
// }

// intro string slice
// for fixed-sized strings
fn intro_to_str_slice() {
    let name: &str = "Sylvia";
    println!("my name is {}", name)
}

fn intro_to_ownable_string() {
    let mut name: String = String::from("Wisdom");
    println!("first name: here: {}", name);
    name.push_str(" John");
    println!("final name: here: {}", name);
    println!("ptr = address in heap memory: {:?}", name.as_ptr());
}

// function to encapsulate all string fn
fn string_data_type_converter() {
    let string = String::from("fan of a fan");
    let string_to_str = string_to_str_string_data_type_converter(&string);
    println!(
        "A String data type of {} is converted to str",
        string_to_str
    );
    //shadow
    let string = "it another beautifull day to see shege banza";
    let string_to_str = str_to_string_string_data_type_converter(string);
    println!(
        "A str data type of {} is converted to string",
        string_to_str
    );
}

// handle all String convertion to str
fn string_to_str_string_data_type_converter(x: &String) -> &str {
    x.as_str()
}

fn str_to_string_string_data_type_converter(x: &str) -> String {
    String::from(x)
}

fn string_formatting(first_name: &str, last_name: &str) -> String {
    let full_name = format!("{} {}", first_name, last_name);
    return full_name;
}
