#[derive(Debug)]
pub enum PaymentMethod {
    Cash,
    CreditCard,
    DebitCard,
    PayPal,
    Cryptocurrency {coin: String, address: String},
}

// Generic type enum
#[derive(Debug)]
enum Options<T> {
    None,
    Some(T)
}

pub fn enum_details() {
    let cash = PaymentMethod::Cash;
    println!("Enum Details: {:?}", cash);

    println!("Options enum {:?}", Options::<i32>::None);
    println!("Option enum {:?}", Options::Some(2));

    let payment = PaymentMethod::CreditCard;

    match payment {
        PaymentMethod::Cash => println!("Payment method is cash"),
        PaymentMethod::CreditCard => println!("Payment method is credit card"),
        PaymentMethod::DebitCard => println!("Payment method is debit card"),
        PaymentMethod::PayPal => println!("Payment method is PayPal"),
        PaymentMethod::Cryptocurrency {coin, address} => {
            println!("Payment method is cryptocurrency");
            println!("Coin: {}", coin);
            println!("Address: {}", address);
        }
    }

    let payment = PaymentMethod::Cryptocurrency {
        coin: String::from("Bitcoin"),
        address: String::from("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa"),
    };

    // Assignment and conditional check using `if let`
    if let PaymentMethod::Cryptocurrency { coin, address } = payment {
        // `coin` and `address` are assigned here
        println!("Payment method is cryptocurrency");
        println!("Coin: {}", coin); // Assigned value of `coin`
        println!("Address: {}", address); // Assigned value of `address`
    } else {
        println!("Not a cryptocurrency payment");
    }
    {

    }
}