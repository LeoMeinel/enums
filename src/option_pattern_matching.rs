/*
 * File: option_pattern_matching.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
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
