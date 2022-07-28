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
 * along with this program. If not, see https://github.com/LeoMeinel/enums/blob/main/LICENSE
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
