/*
 * File: pattern_matching.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

pub use self::enum_pattern_matching::Coin;
pub use self::enum_pattern_matching::UsState;

mod enum_pattern_matching;

pub(crate) fn pattern_matching() {
    let coin: Coin = Coin::Dime(UsState::NewMexico);
    let value = value_in_cents(&coin);
    println!("Value in cents of {:#?} is: {}", coin, value);
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny(_us_state) => 1,
        Coin::Nickel(_us_state) => 5,
        Coin::Dime(_us_state) => 10,
        Coin::Quarter(_us_state) => 25,
    }
}
