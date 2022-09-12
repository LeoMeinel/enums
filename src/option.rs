/*
 * File: option.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

pub use self::enum_option::Option;

mod enum_option;

pub(crate) fn option() {
    let some_number = Option::Some(5);
    let some_string = Option::Some("5");
    let none: Option<i32> = Option::None;
    println!(
        "some_number is: {:#?}, some_string is: {:#?}, none is: {:#?}",
        some_number, some_string, none
    );
}
