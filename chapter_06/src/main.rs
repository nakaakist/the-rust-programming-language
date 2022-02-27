#[derive(Debug)]
enum Coin {
    One,
    Five, // holed
    Ten,
    Fifty, // holed
    OneHundred,
    FiveHundred,
}

fn value_in_coin(coin: &Coin) -> u32 {
    match coin {
        Coin::One => 1,
        Coin::Five => 5,
        Coin::Ten => 10,
        Coin::Fifty => 50,
        Coin::OneHundred => 100,
        Coin::FiveHundred => 500,
    }
}

fn value_in_holed_coin(coin: &Coin) -> Option<u32> {
    match coin {
        Coin::Five => Some(value_in_coin(coin)),
        Coin::Fifty => Some(value_in_coin(coin)),
        _ => None,
    }
}

fn main() {
    let coins = [
        Coin::One,
        Coin::Five,
        Coin::Ten,
        Coin::Fifty,
        Coin::OneHundred,
        Coin::FiveHundred,
    ];

    // calculate total amount of all coins and holed coins
    let mut total = 0;
    let mut holed_total = 0;
    for coin in coins {
        total += value_in_coin(&coin);
        if let Some(value) = value_in_holed_coin(&coin) {
            holed_total += value;
        }
    }

    println!("total: {}", total);
    println!("total of holed coins: {}", holed_total);
}
