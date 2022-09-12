/*
 * File: option_if_let_syntax.rs
 * Author: Leopold Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2022 Leopold Meinel & contributors
 * SPDX ID: GPL-3.0-or-later
 * URL: https://www.gnu.org/licenses/gpl-3.0-standalone.html
 * -----
 */

pub(crate) fn if_let_syntax() {
    equivalent_method();
    if_let_syntax_method();
}

fn if_let_syntax_method() {
    let some_value = Some(3);
    if let Some(3) = some_value {
        println!("three")
    }
}

#[allow(clippy::single_match)]
fn equivalent_method() {
    let some_value = Some(3);
    match some_value {
        Some(3) => println!("three"),
        _ => (),
    }
}
