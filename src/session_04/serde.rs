use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    name: String,
}

pub fn using_serde() -> Result<(), serde_json::Error> {
    let user = User {
        id: 1,
        name: "marto".to_string(),
    };

    rand();

    let user_json = serde_json::to_string(&user)?;
    println!("This is the new {}", user_json);

    let user_de_json: User = serde_json::from_str(&user_json)?;
    println!("This is {:?}", user_de_json);
    Ok(())
}

fn rand() {
    // Generate a random number
    let mut rng = rand::thread_rng();
    let random_num: u32 = rng.gen_range(1..=100);
    println!("Random number: {}", random_num);

    // Shuffle a vector
    let mut numbers = vec![1, 2, 3, 4, 5];
    numbers.shuffle(&mut rng);
    println!("Shuffled: {:?}", numbers);

    // Generate a random boolean
    //The gen() method is generic and can generate
    //random values for any type that implements the
    //rand::distributions::Distribution trait. The
    //rand crate provides an implementation of this
    //trait for many types, including primitive
    //types like bool, u32, f64, etc.

    let coin_flip: bool = rng.gen();
    println!("Coin flip: {}", if coin_flip { "Heads" } else { "Tails" });
}
