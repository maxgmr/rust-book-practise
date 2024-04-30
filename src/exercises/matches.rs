#![allow(unused)]

use rand::Rng;

#[derive(Debug)]
enum CaProvinceTerritory {
    YukonTerritory,
    NorthwestTerritories,
    Nunavut,
    BritishColumbia,
    Alberta,
    Saskatchewan,
    Manitoba,
    Ontario,
    Quebec,
    NewfoundlandAndLabrador,
    NewBrunswick,
    PrinceEdwardIsland,
    NovaScotia,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(CaProvinceTerritory),
    Loonie,
    Toonie,
}

pub fn start() {
    println!(
        "{}",
        value_in_cents(Coin::Penny) + value_in_cents(Coin::Quarter(CaProvinceTerritory::Alberta))
    );

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // roll die. on 3, player doesn't move but gets fancy hat. on 7, player doesn't move and loses fancy hat.
    // else, move that number of spaces.
    // _ => () <- match anything else but don't need to use it, do nothing
    let dice_roll = rand::thread_rng().gen_range(1..=20);
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
}

fn add_fancy_hat() {
    println!("Adding hat! :D");
}
fn remove_fancy_hat() {
    println!("Removing hat :(");
}
fn move_player(num_spaces: u8) {
    println!("Moving player {num_spaces} spaces");
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            let penny_luckiness = rand::thread_rng().gen_range(1..=100);
            if (penny_luckiness == 100) {
                println!("Lucky penny!");
            }
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(province_territory) => {
            println!(
                "Provincial/territorial quarter from {:?}!",
                province_territory
            );
            25
        }
        Coin::Loonie => 100,
        Coin::Toonie => 200,
    }
}
