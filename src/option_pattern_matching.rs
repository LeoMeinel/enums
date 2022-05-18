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

pub(crate) fn option_pattern_matching() {
    call_plus_one();
}

fn call_plus_one() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!(
        "five is: {:#?}, six is: {:#?}, none is: {:#?}",
        five, six, none
    );
    let five = Some(5);
    let six = plus_one_better(five);
    let none = plus_one_better(None);
    println!(
        "five is: {:#?}, six is: {:#?}, none is: {:#?}",
        five, six, none
    )
}

#[allow(clippy::manual_map)]
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        // Any other value
        _ => None,
    }
}

fn plus_one_better(x: Option<i32>) -> Option<i32> {
    x.map(|i| i + 1)
}
