use std::collections::btree_map::Values;

#[derive(Debug)]
enum PaymentMethod {
    Cash,
    Transfer(String),
    Crypto { coin: String, address: String },
}

#[derive(Debug)]
enum Options<T> {
    None,
    Some(T),
}

pub fn enums_details() {
    println!("this are the payment methods {:#?}", PaymentMethod::Cash);
    println!(
        "this are the payment methods {:#?}",
        PaymentMethod::Transfer(String::from("Successful"))
    );
    println!(
        "this are the payment methods {:#?}",
        PaymentMethod::Crypto {
            coin: String::from("S"),
            address: String::from("0x124")
        }
    );

    println!("Options generic enum {:#?}", Options::<i32>::None);
    println!("Options generic enum {:#?}", Options::Some("string"));

    let mut num = 10;

    match num {
        4 => println!("The number is 4"),
        5 | 6 => println!("The number aint equal to 10"),
        10 => num += 1,
        1..=12 => println!("Not equal to 10"),
        _ => println!("numbe:r 10 not found"),
    }

    let payment = PaymentMethod::Crypto {
        coin: "S".to_string(),
        address: "0x124".to_string(),
    };

    if let PaymentMethod::Cash = payment {
        println!("Payment using if let")
    };

    match payment {
        PaymentMethod::Cash => println!("This is {:#?}", PaymentMethod::Cash),
        PaymentMethod::Transfer(val) if val == "Successful".to_string() => println!(
            "This is {:#?}",
            PaymentMethod::Transfer("Successful".to_string())
        ),
        PaymentMethod::Crypto {
            coin: val,
            address: val2,
        } if val == "S" && val2 == "0x124" => println!("Pay with crypto"),
        _ => println!("No payment"),
    }
}
