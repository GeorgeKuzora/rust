fn main() {
    let coin = Coin::Dime;
    println!("Coin value is {}", coin.value_in_cents());
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}
