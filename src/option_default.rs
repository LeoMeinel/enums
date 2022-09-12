/*
 * File: option_default.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

pub(crate) fn default_option() {
    print_values();
    add_values();
}

fn add_values() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y.unwrap();
    let sum_with_default = x + y.unwrap_or(0);
    println!("sum is: {}, sum_with_default is: {}", sum, sum_with_default);
}

fn print_values() {
    let some_number = Some(5);
    let some_string = Some("5");
    let none: Option<i32> = None;
    println!(
        "some_number is: {:#?}, some_string is: {:#?}, none is: {:#?}",
        some_number, some_string, none
    );
}
