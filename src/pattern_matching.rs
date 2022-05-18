/*
 * enums is a commandline application.
 * Copyright © 2022 Leopold Meinel & contributors
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see https://github.com/TamrielNetwork/enums/blob/main/LICENSE
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
